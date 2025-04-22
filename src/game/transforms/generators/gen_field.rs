use crate::game::transforms::terrain::Terrain;

pub fn gen_field(terra: &mut Terrain, depth:i8){
	let size:i64 = (terra.chunk_size_y*terra.chunk_size_x*terra.chunk_size_z).into();

	//see to string method, if size is too big, it cant be printed
	if size > 16i64.pow(4){
		panic!("chunk size too big")
	}

	let mut height_value = 0i8;

	for i in 0..terra.height_map.len() {
		terra.height_map[i] += height_value;
		if terra.height_map[i] > depth.into() {
			terra.height_map[i] = depth.into();
		}
		height_value += 1;
		
		if i % terra.chunk_size_x as usize == 0 {
			height_value = 0;
		}
		if i % ((terra.chunk_size_x as usize) * (terra.chunk_size_y as usize)) == 0 {
			height_value = 0;
		}
	}
}

