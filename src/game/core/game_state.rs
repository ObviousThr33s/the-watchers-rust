use crate::utils::file::{file_object::FileObject, game_file::SaveFile};



pub enum GameStates {
	Exit,
	Init,
	Running
}

struct GameState_ {
	running:bool,
}

pub trait GameState {
	fn new() ->  Self;
	fn update(&mut self);
	fn init(&mut self);

	fn run(&mut self) {
		println!("Game is running...");
	}

	fn exit(&mut self) {
		println!("Game exited!");
	}
}