use crate::utils::file_io::file_data::FileData;

pub mod terrain;
pub mod generators;
pub mod vect;

pub(crate) enum DataTypes{
	Terrain,
	Entity,
	Item,
	File,
	Render,
}

trait Transform {
	fn create_vect_from(dataType:DataTypes) -> FileData;
}
