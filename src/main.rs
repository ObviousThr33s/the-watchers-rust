use the_watchers_rust::game;
use the_watchers_rust::game::game::GameState;
use the_watchers_rust::game::game::GameStates;
use the_watchers_rust::logic;
use the_watchers_rust::utils;
use the_watchers_rust::utils::file_object::FileObject;

pub fn main(){
	let base_game_state:GameState = GameState::new();
	let mut save_file:FileObject = FileObject::new();

	base_game_state.update();

}