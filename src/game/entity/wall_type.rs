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
}