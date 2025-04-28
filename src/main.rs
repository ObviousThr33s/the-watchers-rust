
use the_watchers_rust::loops::main_loop::{self, MainLoop};
use the_watchers_rust::utils::time::Time;


#[tokio::main]
pub async fn main(){
	let start:Time = Time::new();
	let terminal = ratatui::init();
	
	let mut game:main_loop::MainLoop = MainLoop::new(start, terminal);
	

	game.init().await;

}