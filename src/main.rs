
use the_watchers_rust::game_loop::game_loop_main::GameLoopMain;
use the_watchers_rust::utils::logger::Logger;
use the_watchers_rust::utils::{self, time, time::Time};

pub fn main(){
	let start:Time = Time::new();
	let mut game:GameLoopMain = GameLoopMain::new(start);
	

	game.init();

}