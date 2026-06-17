use std::collections::HashSet;

/// Viewport represents the player's view of the world
/// It handles all calculations needed to render the 2.5D perspective
#[derive(Clone)]
pub struct Viewport {
	pub width: usize,
	pub height: usize,
	pub fov: f32,
	pub max_distance: f32,
}

/// Brightness ramp from nearest/brightest to farthest/faintest.
/// Walls facing the viewer along the y-axis are shaded one step darker
/// than x-facing walls so corners read as distinct surfaces.
const RAMP: [char; 5] = ['█', '▓', '▒', '░', '·'];

impl Viewport {
	pub fn new(width: usize, height: usize, fov: f32) -> Self {
		Self {
			width,
			height,
			fov,
			max_distance: 20.0,
		}
	}

	/// Renders a 2.5D ASCII view by casting one ray per screen column.
	/// Returns the rendered frame as a newline-separated string.
	pub fn render_raycasted(&self, player_x: f32, player_y: f32, angle: f32, walls: &[(i16, i16)]) -> String {
		// Build the wall lookup once per frame rather than once per ray.
		let wall_set: HashSet<(i16, i16)> = walls.iter().copied().collect();

		let ray_count = self.width.max(1);
		let angle_step = self.fov / (ray_count as f32);
		let center_column = self.width as f32 / 2.0;
		let center_row = self.height as isize / 2;

		let mut output = vec![vec![' '; self.width]; self.height];

		for column in 0..ray_count {
			// Spread rays evenly across the field of view, centered on `angle`.
			let ray_angle = angle + (column as f32 - center_column) * angle_step;

			let (distance, y_side) = self.cast_ray(player_x, player_y, ray_angle, &wall_set);

			// Fisheye correction: project the hit onto the view direction.
			let corrected = distance * (ray_angle - angle).cos();
			let corrected = corrected.max(0.0001);

			// Standard projection: a wall one cell away fills the view height.
			let line_height = ((self.height as f32) / corrected).round() as isize;
			let half = line_height / 2;
			let wall_top = (center_row - half).max(0) as usize;
			let wall_bottom = (center_row + half).clamp(0, self.height as isize) as usize;

			let wall_char = shade(corrected, y_side, self.max_distance);

			for y in 0..self.height {
				output[y][column] = if y >= wall_top && y < wall_bottom {
					wall_char
				} else if (y as isize) < center_row {
					' ' // ceiling / sky
				} else {
					// Floor gets denser toward the viewer (screen bottom).
					let depth = (y as isize - center_row) as f32 / (center_row.max(1) as f32);
					if depth < 0.5 { '·' } else { ':' }
				};
			}
		}

		output
			.iter()
			.map(|row| row.iter().collect::<String>())
			.collect::<Vec<_>>()
			.join("\n")
	}

	/// Casts a single ray using grid DDA. Returns the distance to the first
	/// wall hit and whether that wall faces the viewer along the y-axis
	/// (`true` => horizontal face, shaded darker; `false` => vertical face).
	///
	/// Cells are centered on integer coordinates (boundaries at the halves),
	/// matching how walls are placed and how positions round to the grid.
	fn cast_ray(&self, start_x: f32, start_y: f32, angle: f32, walls: &HashSet<(i16, i16)>) -> (f32, bool) {
		let dir_x = angle.cos();
		let dir_y = angle.sin();

		// Shift by +0.5 so cell boundaries sit on integers; floor() then
		// matches round() on the original coordinates.
		let px = start_x + 0.5;
		let py = start_y + 0.5;
		let mut map_x = px.floor() as i32;
		let mut map_y = py.floor() as i32;

		// Ray length needed to cross one full cell along each axis.
		let delta_x = if dir_x == 0.0 { f32::INFINITY } else { (1.0 / dir_x).abs() };
		let delta_y = if dir_y == 0.0 { f32::INFINITY } else { (1.0 / dir_y).abs() };

		let (step_x, mut side_x) = if dir_x < 0.0 {
			(-1, (px - map_x as f32) * delta_x)
		} else {
			(1, (map_x as f32 + 1.0 - px) * delta_x)
		};
		let (step_y, mut side_y) = if dir_y < 0.0 {
			(-1, (py - map_y as f32) * delta_y)
		} else {
			(1, (map_y as f32 + 1.0 - py) * delta_y)
		};

		let mut distance = 0.0;
		let mut y_side = false;

		while distance < self.max_distance {
			if side_x < side_y {
				distance = side_x;
				side_x += delta_x;
				map_x += step_x;
				y_side = false;
			} else {
				distance = side_y;
				side_y += delta_y;
				map_y += step_y;
				y_side = true;
			}

			if walls.contains(&(map_x as i16, map_y as i16)) {
				return (distance, y_side);
			}
		}

		(self.max_distance, y_side)
	}
}

/// Picks a shading character from the brightness ramp based on corrected
/// distance, dimming y-facing walls by one step for surface contrast.
fn shade(distance: f32, y_side: bool, max_distance: f32) -> char {
	// Map [0, max_distance] onto the ramp, then darken horizontal faces.
	let span = max_distance.max(1.0);
	let mut band = ((distance / span) * (RAMP.len() as f32)) as usize;
	if y_side {
		band += 1;
	}
	RAMP[band.min(RAMP.len() - 1)]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn viewport_creation() {
		let vp = Viewport::new(80, 24, std::f32::consts::PI / 3.0);
		assert_eq!(vp.width, 80);
		assert_eq!(vp.height, 24);
	}

	#[test]
	fn ray_hits_wall_dead_ahead() {
		// Facing +x from (0,0); a wall at x=3 should be hit at ~3.0 on a
		// vertical (x-facing) surface.
		let vp = Viewport::new(80, 24, std::f32::consts::PI / 3.0);
		let walls: HashSet<(i16, i16)> = [(3, 0)].into_iter().collect();
		let (dist, y_side) = vp.cast_ray(0.0, 0.0, 0.0, &walls);
		assert!((dist - 2.5).abs() < 0.01, "expected near face at 2.5, got {dist}");
		assert!(!y_side, "a wall to the +x side should be an x-facing surface");
	}

	#[test]
	fn ray_misses_into_open_space() {
		let vp = Viewport::new(80, 24, std::f32::consts::PI / 3.0);
		let walls: HashSet<(i16, i16)> = HashSet::new();
		let (dist, _) = vp.cast_ray(0.0, 0.0, 0.0, &walls);
		assert_eq!(dist, vp.max_distance);
	}
}
