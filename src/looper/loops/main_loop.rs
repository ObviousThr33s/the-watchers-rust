

use rand::Rng;

use crate::{game::{entity::{self, Entity}, group::Group}, input::PlayerMove, looper::{self, looper::Looper}};



pub(crate) struct MainLoop {
	event:Box::<()>
}

impl MainLoop {


	pub async fn main_loop(mut entity:Group, player_move:PlayerMove) -> Group{
		let mut rng = rand::rng();

		if let Some(entity) = entity.entities.get_mut("Entity"){
			entity.update(Entity{
				x: rng.random_range(0..50),
				y: rng.random_range(0..30),
				self_: 'E',
				id: "Entity".to_owned(),
			});
		}

		let g = Self::player_move(entity, player_move).await;
		g
	}

	pub async fn player_move(mut entity:Group, player_move:PlayerMove) -> Group{
		if player_move == PlayerMove::UP {
			if let Some(entity) = entity.entities.get_mut("Player") {
				entity.move_up();
			}
		}
		if player_move == PlayerMove::DOWN {
			if let Some(entity) = entity.entities.get_mut("Player") {
				entity.move_down();
			}
		}
		if player_move == PlayerMove::LEFT {
			if let Some(entity) = entity.entities.get_mut("Player") {
				entity.move_left();
			}
		}
		if player_move == PlayerMove::RIGHT {
			if let Some(entity) = entity.entities.get_mut("Player") {
				entity.move_right();
			}
	
		}

		entity
	
	}

}
