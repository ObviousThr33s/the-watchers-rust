use std::collections::HashMap;
use super::pixel::Pixel;
use crate::game::{entity::wall_type::{WallData, WallType}, spaces::field::Field};

pub struct Raster {
    grid: HashMap<(u16, u16), WallData>,
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
			grid.insert((pixel.x as u16, pixel.y as u16), WallData {
				wall_type: WallType::Custom(pixel.char),
				display_char: pixel.char.to_string(),
			});
		}
		
		Self { grid }
	}

	/// Adds a point to the raster
	pub fn add_point(&mut self, x: u16, y: u16, value: WallData) {
		self.grid.insert((x, y), value);
	}

	/// Checks if a point exists in the raster
	pub fn has_point(&self, x: u16, y: u16) -> bool {
		self.grid.contains_key(&(x, y))
	}
	
	/// Gets a value at a specific point if it exists
	pub fn get_point(&self, x: u16, y: u16) -> Option<&WallData> {
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
		self.create_boundary_with_type(width, height, WallType::Custom(border_char.chars().next().unwrap_or('#')));
	}
	
	/// Returns a vector of all points in the raster
	pub fn get_all_points(&self) -> Vec<((u16, u16), &WallData)> {
		self.grid.iter().map(|(pos, val)| (*pos, val)).collect()
	}

	/// Creates a string representation of a field with a ray cast through it
	/// from the specified starting position and angle
	pub fn to_string_with_ray(&self, field:Field, start_x: f32, start_y: f32, angle: f32, max_distance: f32) -> String {
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
						result.push_str(&format!("*{}*", value.display_char));
					} else {
						// Just show the ray path
						result.push_str("·");
					}
				} else if let Some(entity) = field.get_entity_by_position(x as usize, y as usize) {
					// Show the entity without highlighting
					result.push_str(&format!("{}", entity.self_));
				} else if let Some(value) = self.grid.get(&(x, y)) {
					// Show a raster point
					result.push_str(&value.display_char);
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
	pub fn to_2d5_view(&self, start_x: f32, start_y: f32, angle: f32, fov: f32, width: usize, height: usize) -> String {
		// Define constants
		let ray_count = width; // One ray per column
		let half_fov = fov / 2.0;
		let angle_step = fov / (ray_count as f32);
		let max_distance = 20.0; // Maximum viewing distance
		
		// Prepare the output grid
		let mut output = vec![vec![' '; width]; height];
		
		// Cast rays across the field of view
		for column in 0..ray_count {
			// Calculate ray angle
			let ray_angle = self.calculate_ray_angle(angle, half_fov, column, angle_step);
			
			// Get distance to wall and wall type
			let (distance, wall_type) = self.cast_single_ray(start_x, start_y, ray_angle, max_distance);
			
			// Apply fisheye correction
			let corrected_distance = self.correct_fisheye(distance, ray_angle, angle);
			
			// Calculate wall height and position
			let (wall_top, wall_bottom) = self.calculate_wall_dimensions(corrected_distance, height);
			
			// Get wall character based on distance and type
			let wall_char = match wall_type {
				Some(wt) => wt.get_character(corrected_distance),
				None => self.get_default_wall_character(corrected_distance),
			};
			
			// Draw the column
			self.draw_column(&mut output, column, wall_top, wall_bottom, wall_char, height);
		}
		
		// Convert to string
		self.output_to_string(&output)
	}
	
	/// Calculate the angle for a specific ray
	fn calculate_ray_angle(&self, center_angle: f32, half_fov: f32, column: usize, angle_step: f32) -> f32 {
		center_angle - half_fov + (column as f32) * angle_step
	}
	
	/// Cast a single ray and return the distance to a hit
	fn cast_single_ray(&self, start_x: f32, start_y: f32, angle: f32, max_distance: f32) -> (f32, Option<WallType>) {
		let ray_points = self.cast_ray(start_x, start_y, angle, max_distance);
		
		// Find the first obstacle hit by the ray
		if let Some((hit_x, hit_y)) = ray_points.iter().find(|&&(x, y)| self.grid.contains_key(&(x, y))) {
			// Calculate Euclidean distance
			let dx = *hit_x as f32 - start_x;
			let dy = *hit_y as f32 - start_y;
			let distance = (dx * dx + dy * dy).sqrt();
			
			// Get the wall type
			let wall_type = self.grid.get(&(*hit_x, *hit_y)).map(|data| data.wall_type);
			
			(distance, wall_type)
		} else {
			// No obstacle found within range
			(max_distance, None)
		}
	}
	
	/// Apply fisheye correction to distance
	fn correct_fisheye(&self, distance: f32, ray_angle: f32, view_angle: f32) -> f32 {
		let angle_diff = ray_angle - view_angle;
		distance * angle_diff.cos()
	}
	
	/// Calculate wall top and bottom positions
	fn calculate_wall_dimensions(&self, distance: f32, height: usize) -> (usize, usize) {
		// Calculate wall height (inverse to distance)
		let wall_height = (height as f32 * 0.8) / distance.max(0.1);
		let wall_height = wall_height.min(height as f32) as usize;
		
		// Calculate where to start and end drawing the wall
		let wall_top = (height - wall_height) / 2;
		let wall_bottom = wall_top + wall_height;
		
		(wall_top, wall_bottom)
	}
	
	/// Get default wall character based on distance
	fn get_default_wall_character(&self, distance: f32) -> char {
		match distance {
			d if d < 2.0 => '█',
			d if d < 4.0 => '▓',
			d if d < 6.0 => '▒',
			d if d < 10.0 => '░',
			_ => '·',
		}
	}
	
	/// Draw a single column in the output grid
	fn draw_column(&self, output: &mut Vec<Vec<char>>, column: usize, wall_top: usize, wall_bottom: usize, wall_char: char, height: usize) {
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
	
	/// Convert output grid to string
	fn output_to_string(&self, output: &Vec<Vec<char>>) -> String {
		let mut result = String::new();
		for row in output {
			result.push_str(&row.iter().collect::<String>());
			result.push('\n');
		}
		result
	}
	
	// Add a point with a specific wall type
	pub fn add_wall_point(&mut self, x: u16, y: u16, wall_type: WallType) {
		let display_char = wall_type.get_character(0.0).to_string();
		self.grid.insert((x, y), WallData { wall_type, display_char });
	}
	
	// Create boundaries with specific wall types
	pub fn create_boundary_with_type(&mut self, width: u16, height: u16, wall_type: WallType) {
		// Create top and bottom walls
		for x in 0..width {
			self.add_wall_point(x, 0, wall_type);
			self.add_wall_point(x, height - 1, wall_type);
		}
		
		// Create left and right walls
		for y in 0..height {
			self.add_wall_point(0, y, wall_type);
			self.add_wall_point(width - 1, y, wall_type);
		}
	}
}