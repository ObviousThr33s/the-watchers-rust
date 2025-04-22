use crate::game::transforms::terrain::Terrain;

fn gen_field(terra: &mut Terrain){
	let size:i64 = (terra.chunk_size_y*terra.chunk_size_x*terra.chunk_size_z).into();

	//see to string method, if size is too big, it cant be printed
	if size > 16i64.pow(4){
		panic!("chunk size too big")
	}

	



	for _ in 0..size{
			terra.height_map.push(63);
	}
}	

