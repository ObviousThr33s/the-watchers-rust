use crate::{game::entity::player::Player, input::PlayerMove, utils::logger::Logger};


pub struct PlayerLoop{

}

impl PlayerLoop {

	//moves player one single step in a direction, chooses player from entities 
	//list so that player can be moved in place rather than borrow replaced
	pub fn player_move(player:&mut Player, player_move:PlayerMove, logger:&mut Logger) {
		
		if player_move == PlayerMove::UP {
			player.poll_move_forewards();
		}
		if player_move == PlayerMove::DOWN {
			player.poll_move_backwards();
		}
		if player_move == PlayerMove::LEFT {
			player.move_left();
		}
		if player_move == PlayerMove::RIGHT {
			player.move_right();
		}
		
		logger.log(player.player.to_string().as_str());

		logger.log(&format!("{}_deg", player.heading.0));
		
	}
}

