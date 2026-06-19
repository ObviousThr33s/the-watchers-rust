
use the_watchers_rust::game::Game;
use the_watchers_rust::loops::main_loop::MainLoop;
use the_watchers_rust::utils::time::Time;

//Notice how time comes from one place here.

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start: Time = Time::new();
	let terminal = ratatui::init();
	let game:Game = Game::new(/*server stuff?*/);

	let mut game: MainLoop = MainLoop::new(start, terminal, game);

	game.run_game();

	ratatui::restore();
	Ok(())
}