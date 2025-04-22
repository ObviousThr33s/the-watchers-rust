pub struct Terrain {
	pub(crate) height_map:Vec<i16>,
	pub(crate) chunk_size_x:i16,
	pub(crate) chunk_size_y:i16,
	pub(crate) chunk_size_z:i16,

	terrain_type: TerrainTypes
	//like daggerfall, the chunks will be somewhat static
	//mostly encounter based, later on I hope to do
	//low poly deformation based on interaction queues
	//where a chunk will have resources and they can be
	//gathered via a skill check
	//for now, dungeons and crawling, or rather labs since this game is cyberpunk
	

}

#[derive(Clone)]
pub enum TerrainTypes {
	Field,
}

impl Clone for Terrain {
	fn clone(&self) -> Self {
		Terrain {
			height_map: self.height_map.clone(),
			chunk_size_x: self.chunk_size_x,
			chunk_size_y: self.chunk_size_y,
			chunk_size_z: self.chunk_size_z,
			terrain_type: self.terrain_type.clone(),
		}
	}
}

impl Terrain {
	pub fn new(terrain_type:TerrainTypes, ) -> Self{
		Terrain { 
			height_map: Vec::new(),
			chunk_size_x: 10,
			chunk_size_y: 10,
			chunk_size_z: 10,
			terrain_type: terrain_type 
		}
		
	}

	pub fn set_terrain_base(&mut self){
		
	}
	

}

impl ToString for Terrain {
	fn to_string(&self) -> String {
		let _size:i64 = (self.chunk_size_y*self.chunk_size_x*self.chunk_size_z).into();
		
		
		let h_m:Vec<i16> = self.height_map.clone();
		let mut h_m_as_chars:Vec<u8>  = Vec::new();

		for &h in &h_m{
			if h <= 0 && h <= 255 {
				h_m_as_chars.push(h as u8);
			} else {
				h_m_as_chars.push(63u8);
			}
		}

		let s:String = String::from_utf8(h_m_as_chars).expect("Bad char");
		s
	}
}
