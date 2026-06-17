
use the_watchers_rust::loops::main_loop::MainLoop;
use the_watchers_rust::utils::time::Time;

//cool.
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let start: Time = Time::new();
	let terminal = ratatui::init();

	let mut game: MainLoop = MainLoop::new(start, terminal);

	game.run_game();

	ratatui::restore();
	Ok(())
}