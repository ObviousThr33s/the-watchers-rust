
use obelisk::game::Game;
use obelisk::loops::main_loop::MainLoop;
use obelisk::utils::time::Time;

//Notice how time comes from one place here.

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start: Time = Time::new();
	let terminal = ratatui::init();
	let game:Game = Game::new(/*server stuff?*/ );

	//Max:1.0.0
	//Max-1:0.99.39
	let version:String = "0.5.1".to_owned();
	let mut game: MainLoop = MainLoop::new(start, terminal, game, version.to_string());

	game.run_game();

	ratatui::restore();
	Ok(())
}