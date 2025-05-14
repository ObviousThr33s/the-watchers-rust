
use crate::{game::entity::player::{Direction_, Player}, input::PlayerMove, utils::logger::Logger};


pub struct PlayerLoop{

}

impl PlayerLoop {

	//moves player one single step in a direction, chooses player from entities 
	//list so that player can be moved in place rather than borrow replaced
	pub fn player_move(player:&mut Player, player_move:PlayerMove, logger:&mut Logger) {
		
		if player_move == PlayerMove::LEFT {
			player.add_direction(90.0);
		}else if player_move == PlayerMove::RIGHT {
			player.sub_direction(90.0);
		}else if player_move == PlayerMove::NONE {
			return;
		}
		
		if player_move == PlayerMove::UP {
			if player.direction == Direction_::UP {
				player.player.y -= 1;
			}if player.direction == Direction_::DOWN {
				player.player.y += 1;
			}if player.direction == Direction_::LEFT {
				player.player.x -= 1;
			}if player.direction == Direction_::RIGHT {
				player.player.x += 1;
			}
		}

		if player_move == PlayerMove::DOWN {
			if player.direction == Direction_::DOWN {
				player.player.y -= 1;
			}if player.direction == Direction_::UP {
				player.player.y += 1;
			}if player.direction == Direction_::RIGHT {
				player.player.x -= 1;
			}if player.direction == Direction_::LEFT {
				player.player.x += 1;
			}
		}
		
		
		logger.log(player.player.to_string().as_str());

		logger.log(&format!("{}_deg", player.heading.0));
		
	}
}

