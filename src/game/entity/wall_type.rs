use super::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallType {
	Stone,
	Wood,
	Metal,
	Glass,
	Brick,
	Custom(char),
}

pub struct WallData {
	pub wall_type: WallType,
	pub(crate) display_char: String,
}

impl Clone for WallData {
	fn clone(&self) -> Self {
		Self {
			wall_type: self.wall_type.clone(),
			display_char: self.display_char.clone(),
		}
	}
}

impl WallType {
	pub fn get_character(&self, distance: f32) -> char {
		match self {
			WallType::Stone => match distance {
				d if d < 2.0 => '■',
				d if d < 4.0 => '▓',
				d if d < 6.0 => '▒',
				d if d < 10.0 => '░',
				_ => '·',
			},
			WallType::Wood => match distance {
				d if d < 2.0 => '#',
				d if d < 4.0 => '%',
				d if d < 6.0 => '+',
				d if d < 10.0 => '-',
				_ => '·',
			},
			WallType::Metal => match distance {
				d if d < 2.0 => '█',
				d if d < 4.0 => '▓',
				d if d < 6.0 => '▒',
				d if d < 10.0 => '░',
				_ => '·',
			},
			WallType::Glass => match distance {
				d if d < 2.0 => '○',
				d if d < 4.0 => '◌',
				d if d < 6.0 => '·',
				d if d < 10.0 => '.',
				_ => ' ',
			},
			WallType::Brick => match distance {
				d if d < 2.0 => '█',
				d if d < 4.0 => '▒',
				d if d < 6.0 => '░',
				d if d < 10.0 => '·',
				_ => ' ',
			},
			WallType::Custom(c) => *c,
		}
	}
	
	pub fn from_str(s: &str) -> Self {
		match s {
			"stone" => WallType::Stone,
			"wood" => WallType::Wood,
			"metal" => WallType::Metal,
			"glass" => WallType::Glass,
			"brick" => WallType::Brick,
			_ => WallType::Custom('#'),
		}
	}

	pub fn determine_wall_type(entity: &Entity) -> WallType {
		// Determine wall type based on entity character or ID
		match entity.self_ {
			'#' => WallType::Stone,
			'+' => WallType::Wood,
			'M' => WallType::Metal,
			'G' => WallType::Glass,
			'B' => WallType::Brick,
			// For entities with other characters, use their ID to determine type
			_ => match entity.id.as_str() {
				id if id.contains("wall") => WallType::Stone,
				id if id.contains("wood") => WallType::Wood,
				id if id.contains("metal") => WallType::Metal,
				id if id.contains("glass") => WallType::Glass, 
				id if id.contains("brick") => WallType::Brick,
				// Default to a custom wall with the entity's character
				_ => WallType::Custom(entity.self_),
			},
		}
	}
}