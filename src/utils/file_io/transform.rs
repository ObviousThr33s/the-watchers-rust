struct Transform {
	data_type:DataTypes
}

enum DataTypes{
	Terrain
}

trait Transform {
	fn write_to_file(data_type:DataTypes, data:_);
}


impl Transform for TerrainData {
	fn write_to_file(data_type:DataTypes, data:Terrain){
		
	}
}
