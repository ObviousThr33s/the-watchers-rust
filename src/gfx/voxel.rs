//! The voxel renderer — altitude turned into an ASCII view, Voxel-Space style.
//!
//! Where [`Viewport`](crate::gfx::Viewport) casts flat walls (one height, Doom's
//! limit), this reads a [`Heightmap`] and draws *relief*: one ray per screen
//! column, marched near to far, projecting each ground sample's height onto the
//! column, with a per-column y-buffer so nearer or taller land hides whatever
//! stands behind it. Hidden-surface removal falls out of the march for free — no
//! depth buffer, no overdraw — which is what keeps it cheap enough to draw one
//! frame per move and not a stroke more.

use crate::game::spaces::heightmap::Heightmap;

/// Brightness ramp, nearest/brightest to farthest/faintest — the same depth cue
/// the flat viewport uses, so the two views read as one world.
const RAMP: [char; 5] = ['█', '▓', '▒', '░', '·'];

/// Where eye level sits down the screen, as a fraction of its height. Low (toward
/// the bottom) hands most of the panel to relief and distance and keeps the ground
/// at your feet from eating the frame — you look out over the land, not down at it.
const HORIZON_FRAC: f32 = 0.5;

/// The first-person voxel camera. Holds only framing and the eye's height above
/// the ground datum; the land arrives as a [`Heightmap`] at render time and is
/// never stored here — so one camera renders a fixture, a noise field, or a
/// streamed chunk without knowing which.
#[derive(Clone)]
pub struct Voxel {
	pub width: usize,
	pub height: usize,
	pub fov: f32,
	/// How far a ray marches before it gives up on finding ground.
	pub max_distance: f32,
	/// How high the eye rides above the ground it stands on. The camera plants
	/// itself on the terrain under the player each frame, so this is a height
	/// *above local ground*, not an absolute altitude — ground that rises above the
	/// eye towers over the horizon, ground below it falls away as floor.
	pub eye_height: f32,
	/// Vertical projection gain — how many screen rows a unit of relief spans at
	/// unit distance. Sized to the view by default.
	pub scale: f32,
}

impl Voxel {
	pub fn new(width: usize, height: usize, fov: f32) -> Self {
		Self {
			width,
			height,
			fov,
			max_distance: 32.0,
			eye_height: 4.0,
			scale: height as f32 * 0.5,
		}
	}

	/// Render `terrain` from `(px, py)` looking along `angle` (radians; `0` = `+x`,
	/// `+y` down — the ray-caster convention). One column march each, near to far,
	/// with a y-buffer per column; the frame comes back as newline-joined rows.
	pub fn render(&self, px: f32, py: f32, angle: f32, terrain: &impl Heightmap) -> String {
		let mut output = vec![vec![' '; self.width]; self.height];

		let center_column = self.width as f32 / 2.0;
		let angle_step = self.fov / self.width.max(1) as f32;
		let horizon = self.height as f32 * HORIZON_FRAC;

		// The camera stands on the ground under the player and looks out from
		// `eye_height` above it, so relief is read relative to where you stand —
		// not from a fixed altitude that would bury you in any tall terrain.
		let here_x = (px + 0.5).floor() as i16;
		let here_y = (py + 0.5).floor() as i16;
		let eye = terrain.height(here_x, here_y) as f32 + self.eye_height;

		for column in 0..self.width {
			let ray_angle = angle + (column as f32 - center_column) * angle_step;
			let dir_x = ray_angle.cos();
			let dir_y = ray_angle.sin();
			// Perpendicular distance, so a straight ridge reads straight (no fisheye bow).
			let cos_off = (ray_angle - angle).cos();

			// `y_buffer` is the topmost row already filled in this column; it starts
			// at the screen bottom (nothing drawn). Nearer land fills down to it and
			// lowers it, so farther land can only ever add what pokes *above* — that
			// is the whole of the occlusion, paid for once per sample.
			let mut y_buffer = self.height;

			let mut t = 0.0_f32;
			while t < self.max_distance {
				let wx = px + dir_x * t;
				let wy = py + dir_y * t;
				// +0.5 then floor rounds to the nearest cell, matching how positions
				// snap to the grid elsewhere.
				let cell_x = (wx + 0.5).floor() as i16;
				let cell_y = (wy + 0.5).floor() as i16;
				let h = terrain.height(cell_x, cell_y) as f32;

				let dist = (t * cos_off).max(0.0001);
				let top_f = horizon + (eye - h) / dist * self.scale;
				let top = top_f.round().clamp(0.0, self.height as f32) as usize;

				if top < y_buffer {
					let ch = shade(dist, self.max_distance);
					for row in output.iter_mut().take(y_buffer).skip(top) {
						row[column] = ch;
					}
					y_buffer = top;
					if y_buffer == 0 {
						break; // the column is full to the sky — nothing farther can show
					}
				}

				t += 0.2;
			}
		}

		output
			.iter()
			.map(|row| row.iter().collect::<String>())
			.collect::<Vec<_>>()
			.join("\n")
	}
}

/// Pick a ramp glyph from corrected distance — near is solid, far fades out.
fn shade(dist: f32, max_distance: f32) -> char {
	let span = max_distance.max(1.0);
	let band = ((dist / span) * RAMP.len() as f32) as usize;
	RAMP[band.min(RAMP.len() - 1)]
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f32::consts::PI;

	/// A small camera at the origin looking along `+x` — the shared rig. Odd height
	/// keeps the horizon on a whole row (11 / 2 → row 5).
	fn camera() -> Voxel {
		Voxel::new(21, 11, PI / 3.0)
	}

	fn row(frame: &str, n: usize) -> String {
		frame.lines().nth(n).unwrap_or_default().to_owned()
	}

	/// FINISH LINE 1 — altitude reads as height. A ridge standing ahead fills the
	/// sky above the horizon that flat ground leaves blank; that lift is the whole
	/// reason a heightmap renderer exists over the flat caster.
	#[test]
	fn altitude_rises_above_the_horizon() {
		let cam = camera();
		let flat = |_x: i16, _y: i16| 0u8;
		let ridge = |x: i16, _y: i16| if x == 3 { 10 } else { 0 };

		let on_flat = cam.render(0.0, 0.0, 0.0, &flat);
		let on_ridge = cam.render(0.0, 0.0, 0.0, &ridge);

		assert_ne!(on_flat, on_ridge, "a ridge must change the view");

		// A row well above the horizon (horizon = 5), read at the centre column.
		let above = 2;
		let mid = cam.width / 2;
		let flat_cell = row(&on_flat, above).chars().nth(mid).unwrap_or(' ');
		let ridge_cell = row(&on_ridge, above).chars().nth(mid).unwrap_or(' ');
		assert_eq!(flat_cell, ' ', "flat ground leaves the sky blank above the horizon");
		assert_ne!(ridge_cell, ' ', "the ridge rises into that sky");
	}

	/// FINISH LINE 2 — near occludes far. A low bump tucked directly behind a tall
	/// ridge changes nothing on screen: the ridge's y-buffer has already claimed
	/// those rows, so the bump is never even sampled. What is hidden cannot be seen.
	#[test]
	fn a_ridge_hides_what_stands_behind_it() {
		let cam = camera();
		let ridge = |x: i16, _y: i16| if x == 3 { 10 } else { 0 };
		let ridge_and_bump = |x: i16, _y: i16| match x {
			3 => 10,
			7 => 3,
			_ => 0,
		};

		let without = cam.render(0.0, 0.0, 0.0, &ridge);
		let with = cam.render(0.0, 0.0, 0.0, &ridge_and_bump);

		assert!(without.contains('█'), "the ridge itself must actually render");
		assert_eq!(without, with, "a feature behind the ridge cannot change the frame");
	}
}
