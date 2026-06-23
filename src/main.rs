
use obelisk::game::Game;
use obelisk::loops::main_loop::MainLoop;
use obelisk::utils::time::Time;

//Notice how time comes from one place here.

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start: Time = Time::new();
	let terminal = ratatui::init();
	let game:Game = Game::new(/*server stuff?*/ );

	let mut game: MainLoop = MainLoop::new(start, terminal, game, env!("CARGO_PKG_VERSION").to_string());

	game.run_game();

	ratatui::restore();
	Ok(())
}