use crate::game::spaces::{field::Field, room::Room};

pub(crate) struct WolrdLoop{}

impl WolrdLoop {
	pub fn init(field:&mut Field) {
		//generate local entities
		let field_ = field.gen_entities(field.clone().entities);

		//messy clone into some other systems self
		field.entities = field_.clone();
		
		//generate the floor itself
		let floor = Room::gen_floor(5, 5, 20, 20, "FLOOR".to_owned());
		
		//put the entities on the floor
		for i in floor{
			field.entities.entities.insert(i.id.clone(), i);
		}
	}
}