use crate::input::PlayerMove;

use super::Entity;

pub struct Player {
	pub player:Entity
}

impl Clone for Player {
	fn clone(&self) -> Self {
		Self { player: self.player.clone() }
	}
}

impl Player {
	pub fn player_mover(mut self, player_move:PlayerMove) {
		if player_move == PlayerMove::UP {
			self.player.move_up();
		}
		if player_move == PlayerMove::DOWN {
			self.player.move_down();
		}
		if player_move == PlayerMove::LEFT {
			self.player.move_left();
		}
		if player_move == PlayerMove::RIGHT {
			self.player.move_right();
		}
	}
}
