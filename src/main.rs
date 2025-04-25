
use the_watchers_rust::looper::looper::{self, Looper};
use the_watchers_rust::utils::time::Time;


#[tokio::main]
pub async fn main(){
	let start:Time = Time::new();
	let terminal = ratatui::init();
	
	let mut game:looper::Looper = Looper::new(start, terminal);
	

	game.init().await;

}