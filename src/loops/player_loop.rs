use crate::{game::group::Group, input::PlayerMove, utils::logger::{Logger}};

pub struct PlayerLoop{

}

impl PlayerLoop {

	pub fn is_colliding(_entity:&mut Group){

	}

	pub fn player_move(entity:&mut Group, player_move:PlayerMove, logger:&mut Logger) -> Option<Group>{
		if let Some(entity) = entity.entities.get_mut("Player"){
			if player_move == PlayerMove::UP {
					entity.move_up();
					logger.log(entity.to_string().as_str());
			}
			if player_move == PlayerMove::DOWN {
					entity.move_down();
					logger.log(entity.to_string().as_str());
			}
			if player_move == PlayerMove::LEFT {
					entity.move_left();
					logger.log(entity.to_string().as_str());
			}
			if player_move == PlayerMove::RIGHT {
					entity.move_right();
					logger.log(entity.to_string().as_str());
			}
			
		}
		
		Some(entity.to_owned())
	}
}

