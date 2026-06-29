//! The voxel renderer — altitude turned into a Braille view, Voxel-Space style.
//!
//! Where [`Viewport`](crate::gfx::Viewport) casts flat walls (one height, Doom's
//! limit), this reads a [`Heightmap`] and draws *relief*: one ray per Braille
//! sub-column, marched near to far, projecting each ground sample's height onto
//! the column, with a per-column y-buffer so nearer or taller land hides whatever
//! stands behind it. Hidden-surface removal falls out of the march for free — no
//! depth buffer, no overdraw — which is what keeps it cheap enough to draw one
//! frame per move and not a stroke more.

use crate::game::spaces::heightmap::Heightmap;

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
		// Braille gives every cell a 2x4 grid of dots, so the view is built at a
		// sub-cell resolution of `(2*width) x (4*height)` and only packed down to
		// glyphs at the very end — eight times the detail of a cell-block fill, which
		// is what lets a skyline read as a curve instead of a staircase. Dots are
		// on/off only; depth shading and material return later, carried by colour.
		let sub_w = self.width * 2;
		let sub_h = self.height * 4;
		let mut dots = vec![false; sub_w * sub_h];

		let center = sub_w as f32 / 2.0;
		let angle_step = self.fov / sub_w.max(1) as f32;
		let horizon = sub_h as f32 * HORIZON_FRAC;
		// `scale` is in cell-rows; the sub-grid is four times finer, so the
		// projection scales up to match and the geometry is unchanged.
		let scale = self.scale * 4.0;

		// The camera stands on the ground under the player and looks out from
		// `eye_height` above it, so relief is read relative to where you stand —
		// not from a fixed altitude that would bury you in any tall terrain.
		let here_x = (px + 0.5).floor() as i16;
		let here_y = (py + 0.5).floor() as i16;
		let eye = terrain.height(here_x, here_y) as f32 + self.eye_height;

		// One ray per sub-column — twice the columns of the cell grid.
		for sx in 0..sub_w {
			let ray_angle = angle + (sx as f32 - center) * angle_step;
			let dir_x = ray_angle.cos();
			let dir_y = ray_angle.sin();
			// Perpendicular distance, so a straight ridge reads straight (no fisheye bow).
			let cos_off = (ray_angle - angle).cos();

			// `y_buffer` is the topmost sub-row already filled in this column; it
			// starts at the screen bottom (nothing drawn). Nearer land fills down to
			// it and lowers it, so farther land can only ever add what pokes *above* —
			// that is the whole of the occlusion, paid for once per sample.
			let mut y_buffer = sub_h;

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
				let top_f = horizon + (eye - h) / dist * scale;
				let top = top_f.round().clamp(0.0, sub_h as f32) as usize;

				if top < y_buffer {
					for sy in top..y_buffer {
						dots[sy * sub_w + sx] = true;
					}
					y_buffer = top;
					if y_buffer == 0 {
						break; // the column is full to the sky — nothing farther can show
					}
				}

				t += 0.2;
			}
		}

		pack_braille(&dots, sub_w, self.width, self.height)
	}
}

/// Pack a `(2*width) x (4*height)` dot grid down to one Braille glyph per cell.
/// A cell with no dots set comes back as a plain space, so empty sky stays bare
/// paper rather than a wall of blank-Braille glyphs.
fn pack_braille(dots: &[bool], sub_w: usize, width: usize, height: usize) -> String {
	// Dot -> bit within a Braille cell (the glyph is U+2800 + the set bits),
	// indexed `[row][col]` over the cell's 2-wide, 4-tall dot grid.
	const BIT: [[u8; 2]; 4] = [
		[0x01, 0x08],
		[0x02, 0x10],
		[0x04, 0x20],
		[0x40, 0x80],
	];

	let mut out = String::with_capacity((width + 1) * height);
	for cy in 0..height {
		for cx in 0..width {
			let mut bits = 0u8;
			for (dy, row) in BIT.iter().enumerate() {
				for (dx, bit) in row.iter().enumerate() {
					let sy = cy * 4 + dy;
					let sx = cx * 2 + dx;
					if dots[sy * sub_w + sx] {
						bits |= bit;
					}
				}
			}
			out.push(match bits {
				0 => ' ',
				_ => char::from_u32(0x2800 + u32::from(bits)).unwrap_or(' '),
			});
		}
		if cy + 1 < height {
			out.push('\n');
		}
	}
	out
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

		let drew_something = without.chars().any(|c| c != ' ' && c != '\n');
		assert!(drew_something, "the ridge itself must actually render");
		assert_eq!(without, with, "a feature behind the ridge cannot change the frame");
	}
}
