use std::collections::HashMap;
use super::pixel::Pixel;
use crate::game::spaces::field::Field;

pub struct Raster {
	grid: HashMap<(u16, u16), String>
}

impl Clone for Raster {
	fn clone(&self) -> Self {
		Self { grid: self.grid.clone() }
	}
}

impl Raster {
	/// Creates a new empty Raster
	pub fn new() -> Self {
		Self {
			grid: HashMap::new()
		}
	}

	/// Creates a Raster from a collection of Pixels
	pub fn from_pixels(pixels: Vec<Pixel>) -> Self {
		let mut grid = HashMap::new();
		
		// Convert pixels to grid entries
		for pixel in pixels {
			grid.insert((pixel.x as u16, pixel.y as u16), pixel.char.to_string());
		}
		
		Self { grid }
	}

	/// Adds a point to the raster
	pub fn add_point(&mut self, x: u16, y: u16, value: String) {
		self.grid.insert((x, y), value);
	}
	
	/// Checks if a point exists in the raster
	pub fn has_point(&self, x: u16, y: u16) -> bool {
		self.grid.contains_key(&(x, y))
	}
	
	/// Gets a value at a specific point if it exists
	pub fn get_point(&self, x: u16, y: u16) -> Option<&String> {
		self.grid.get(&(x, y))
	}

	/// Clears all points from the raster
	pub fn clear(&mut self) {
		self.grid.clear();
	}

	/// Casts a ray from a starting point in a given direction
	pub fn cast_ray(&self, start_x: f32, start_y: f32, angle: f32, max_distance: f32) -> Vec<(u16, u16)> {
		let mut points = Vec::new();
		
		// Calculate direction vector from angle
		let direction_x = angle.cos();
		let direction_y = angle.sin();
		
		// Step size for the ray
		let step_size = 0.1;
		let mut distance = 0.0;
		
		while distance < max_distance {
			// Calculate current position
			let x = start_x + direction_x * distance;
			let y = start_y + direction_y * distance;
			
			// Convert to grid coordinates
			let grid_x = x.round() as u16;
			let grid_y = y.round() as u16;
			
			// Add point to result if it's a new cell
			if points.is_empty() || points.last() != Some(&(grid_x, grid_y)) {
				points.push((grid_x, grid_y));
				
				// Check if we hit something in the grid
				if self.grid.contains_key(&(grid_x, grid_y)) {
					break;
				}
			}
			
			distance += step_size;
		}
		
		points
	}
	
	/// Creates a simple rectangular boundary in the raster
	pub fn create_boundary(&mut self, width: u16, height: u16, border_char: &str) {
		// Create top and bottom walls
		for x in 0..width {
			self.add_point(x, 0, border_char.to_string());
			self.add_point(x, height - 1, border_char.to_string());
		}
		
		// Create left and right walls
		for y in 0..height {
			self.add_point(0, y, border_char.to_string());
			self.add_point(width - 1, y, border_char.to_string());
		}
	}
	
	/// Returns a vector of all points in the raster
	pub fn get_all_points(&self) -> Vec<((u16, u16), &String)> {
		self.grid.iter().map(|(pos, val)| (*pos, val)).collect()
	}

	/// Creates a string representation of a field with a ray cast through it
	/// from the specified starting position and angle
	pub fn to_string_with_ray(&self, field: &Field, start_x: f32, start_y: f32, angle: f32, max_distance: f32) -> String {
		// Get the dimensions of the area we need to represent
		let mut min_x = u16::MAX;
		let mut min_y = u16::MAX;
		let mut max_x = 0;
		let mut max_y = 0;
		
		// Include all points in our grid
		for &(x, y) in self.grid.keys() {
			min_x = min_x.min(x);
			min_y = min_y.min(y);
			max_x = max_x.max(x);
			max_y = max_y.max(y);
		}
		
		// Cast the ray and include those points in our dimensions
		let ray_points = self.cast_ray(start_x, start_y, angle, max_distance);
		for &(x, y) in &ray_points {
			min_x = min_x.min(x);
			min_y = min_y.min(y);
			max_x = max_x.max(x);
			max_y = max_y.max(y);
		}
		
		// Add a small buffer around the edges
		min_x = min_x.saturating_sub(1);
		min_y = min_y.saturating_sub(1);
		max_x += 1;
		max_y += 1;
		
		// Create a set of ray point coordinates for quick lookup
		let ray_set: std::collections::HashSet<(u16, u16)> = ray_points.into_iter().collect();
		
		// Build the string representation
		let mut result = String::new();
		
		// Add a header
		result.push_str(&format!("Ray cast from ({:.1}, {:.1}) at angle {:.2} radians:\n", 
								 start_x, start_y, angle));
		
		// Create the grid representation
		for y in min_y..=max_y {
			for x in min_x..=max_x {
				// Check if this coordinate is in the ray's path
				if ray_set.contains(&(x, y)) {
					// Check if we have an entity from the field at this position
					if let Some(entity) = field.get_entity_by_position(x as usize, y as usize) {
						// Show the entity with highlighting (using *)
						result.push_str(&format!("*{}*", entity.self_));
					} else if let Some(value) = self.grid.get(&(x, y)) {
						// Show a raster point with highlighting
						result.push_str(&format!("*{}*", value));
					} else {
						// Just show the ray path
						result.push_str("·");
					}
				} else if let Some(entity) = field.get_entity_by_position(x as usize, y as usize) {
					// Show the entity without highlighting
					result.push_str(&format!("{}", entity.self_));
				} else if let Some(value) = self.grid.get(&(x, y)) {
					// Show a raster point
					result.push_str(value);
				} else {
					// Empty space
					result.push(' ');
				}
			}
			result.push('\n');
		}
		
		result
	}

	/// Creates a 2.5D representation of the world from the player's perspective
	pub fn to_2d5_view(&self, field: &Field, start_x: f32, start_y: f32, angle: f32, fov: f32, width: usize, height: usize) -> String {
		// Define constants
		let ray_count = width; // One ray per column
		let half_fov = fov / 2.0;
		let angle_step = fov / (ray_count as f32);
		let max_distance = 20.0; // Maximum viewing distance
		
		// Prepare the output grid
		let mut output = vec![vec![' '; width]; height];
		
		// Cast rays across the field of view
		for column in 0..ray_count {
			// Calculate ray angle (angle - half_fov + column * angle_step)
			let ray_angle = angle - half_fov + (column as f32) * angle_step;
			
			// Cast a ray and find the first obstacle
			let ray_points = self.cast_ray(start_x, start_y, ray_angle, max_distance);
			
			// Calculate distance to first obstacle (if any)
			let distance = if let Some((hit_x, hit_y)) = ray_points.iter().find(|&&(x, y)| self.grid.contains_key(&(x, y))) {
				let dx = *hit_x as f32 - start_x;
				let dy = *hit_y as f32 - start_y;
				(dx * dx + dy * dy).sqrt() 
			} else {
				max_distance // No obstacle found within range
			};
			
			// Apply fisheye correction (multiply distance by cos of angle difference)
			let angle_diff = ray_angle - angle;
			let corrected_distance = distance * angle_diff.cos();
			
			// Calculate wall height (inverse to distance)
			let wall_height = (height as f32 * 0.8) / corrected_distance.max(0.1);
			let wall_height = wall_height.min(height as f32) as usize;
			
			// Calculate where to start and end drawing the wall
			let wall_top = (height - wall_height) / 2;
			let wall_bottom = wall_top + wall_height;
			
			// Choose character/shading based on distance
			let wall_char = match corrected_distance {
				d if d < 2.0 => '█',
				d if d < 4.0 => '▓',
				d if d < 6.0 => '▒',
				d if d < 10.0 => '░',
				_ => '·',
			};
			
			// Draw the wall column
			for y in 0..height {
				output[y][column] = if y >= wall_top && y < wall_bottom {
					wall_char
				} else if y < wall_top {
					' ' // Sky
				} else {
					'.' // Ground
				};
			}
		}
		
		// Convert to string
		let mut result = String::new();
		for row in output {
			result.push_str(&row.iter().collect::<String>());
			result.push('\n');
		}
		
		result
	}
}