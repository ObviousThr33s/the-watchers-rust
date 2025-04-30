use std::f64::consts::E;

use crate::{game::{entity::player::Player, group::Group}, input::PlayerMove, utils::logger::Logger};

pub struct PlayerLoop{

}

impl PlayerLoop {

	//unused at the moment but indicates type usage paradigm for now
	pub fn is_colliding(_entity:&mut Group){
	}

	//moves player one single step in a direction, chooses player from entities 
	//list so that player can be moved in place rather than borrow replaced
	pub fn player_move(entity:&mut Group, mut player: &mut Player, player_move:PlayerMove, logger:&mut Logger) -> Option<Group>{
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
					player.move_left(entity);
					logger.log(entity.to_string().as_str());
			}
			if player_move == PlayerMove::RIGHT {
					entity.self_ = '>';
					player.move_right(entity);
					logger.log(entity.to_string().as_str());
			}
		}

		logger.log(&format!("{}_deg", player.direction.0));
		
		Some(entity.to_owned())
	}
}

