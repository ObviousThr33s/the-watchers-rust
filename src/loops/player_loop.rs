
use crate::{
	game::{entity::player::{Direction_, Player}, spaces::field::Field},
	input::PlayerMove,
	utils::logger::Logger,
};

pub struct PlayerLoop{

}

impl PlayerLoop {

	/// Applies one input to the player: `LEFT`/`RIGHT` rotate in place, while
	/// `UP`/`DOWN` step forward/backward along the current facing. Translation
	/// is blocked when the destination cell is occupied, so the player can no
	/// longer walk through walls (or any other entity).
	pub fn player_move(player:&mut Player, player_move:PlayerMove, field:&Field, logger:&mut Logger) {

		match player_move {
			PlayerMove::LEFT  => { player.add_direction(90.0); return; }
			PlayerMove::RIGHT => { player.sub_direction(90.0); return; }
			PlayerMove::NONE  => return,
			PlayerMove::UP | PlayerMove::DOWN => {}
		}

		// Step forward when UP, backward when DOWN.
		let (fx, fy) = Self::forward_delta(&player.direction);
		let (dx, dy) = if player_move == PlayerMove::UP { (fx, fy) } else { (-fx, -fy) };

		let (x, y) = player.player.get_position();
		let (nx, ny) = (x + dx, y + dy);

		if field.is_occupied(nx, ny, &player.player.id) {
			logger.log(&format!("Blocked at ({}, {})", nx, ny));
			return;
		}

		player.player.set_position(nx, ny);

		logger.log(player.player.to_string().as_str());
		logger.log(&format!("{}_deg", player.heading.0));
	}

	/// Unit step vector for the direction the player faces. Screen coordinates:
	/// +x is right, +y is down, so "up" is -y.
	fn forward_delta(direction:&Direction_) -> (i16, i16) {
		match direction {
			Direction_::UP    => (0, -1),
			Direction_::DOWN  => (0,  1),
			Direction_::LEFT  => (-1, 0),
			Direction_::RIGHT => ( 1, 0),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::{Entity, Priority};
	use crate::utils::time::Time;

	fn logger() -> Logger {
		Logger::new(Time::new(), "test".to_string())
	}

	#[test]
	fn walks_into_open_space() {
		// Player starts at (2, 2) facing UP; forward is (2, 1).
		let mut player = Player::new();
		let field = Field::new();
		PlayerLoop::player_move(&mut player, PlayerMove::UP, &field, &mut logger());
		assert_eq!(player.player.get_position(), (2, 1));
	}

	#[test]
	fn blocked_by_wall_ahead() {
		let mut player = Player::new();
		let mut field = Field::new();
		field.add_entity(Entity::new(2, 1, '#', "wall".to_string(), Priority::LOW));
		PlayerLoop::player_move(&mut player, PlayerMove::UP, &field, &mut logger());
		assert_eq!(player.player.get_position(), (2, 2), "should not move into a wall");
	}

	#[test]
	fn rotation_changes_facing_not_position() {
		let mut player = Player::new();
		let field = Field::new();
		PlayerLoop::player_move(&mut player, PlayerMove::RIGHT, &field, &mut logger());
		assert_eq!(player.player.get_position(), (2, 2));
		assert_eq!(player.direction, Direction_::RIGHT);
	}
}
