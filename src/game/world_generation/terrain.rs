struct Terrain {
	height_map:Vec<i16>,
	chunk_size_x:i16,
	chunk_size_y:i16,
	chunk_size_z:i16,

	terrain_type: TerrainTypes
	//like daggerfall, the chunks will be somewhat static
	//mostly encounter based, later on I hope to do
	//low poly deformation based on interaction queues
	//where a chunk will have resources and they can be
	//gathered via a skill check
	//for now, dungeons and crawling, or rather labs since this game is cyberpunk
}

enum TerrainTypes {
	Field,
}

impl Terrain {
	fn new(terrain_type:TerrainTypes, ) -> Self{
		Terrain { 
			height_map: Vec::new(),
			chunk_size_x: 0,
			chunk_size_y: 0,
			chunk_size_z: 0,
			terrain_type: terrain_type 
		}
		
	}

	fn make_terrain_base(&mut self, terrain_type:TerrainTypes){
		match terrain_type {
			TerrainTypes::Field => Self::gen_field(self)
		}

	}

	fn gen_field(&mut self){
		let size:i64 = (self.chunk_size_y*self.chunk_size_x*self.chunk_size_z).into();

		//see to string method, if size is too big, it cant be printed
		if size > 16i64.pow(4){
			panic!("chunk size too big")
		}

		for _ in 0..size{
			self.height_map.push(0);
		}
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