
use the_watchers_rust::game::core::game_loop::{GameLoop, Loop};

pub fn main(){
	let mut base_game_state:GameLoop = GameLoop::new();

	//let mut save_file:FileObject = FileObject::new();

	base_game_state.init();

}