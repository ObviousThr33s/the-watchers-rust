
enum DataTypes{
	Terrain
}

trait Transform {
	const data_type:DataTypes;	
	fn write_to_file(data:DataTypes);
}



impl Transform for TerrainData {
	fn write_to_file(data:DataTypes){
		
	}
	
	const data_type:DataTypes;
}
