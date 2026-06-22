
use obelisk::game::Game;
use obelisk::loops::main_loop::MainLoop;
use obelisk::utils::time::Time;

//Notice how time comes from one place here.

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start: Time = Time::new();
	let terminal = ratatui::init();
	let game:Game = Game::new(/*server stuff?*/ );

	let mut game: MainLoop = MainLoop::new(start, terminal, game, "0.6.0".to_string());

	game.run_game();

	ratatui::restore();
	Ok(())
}