
use ratatui::DefaultTerminal;
use crate::game::Game;
use crate::gfx::Viewport;
use crate::gfx::portal::Portal;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::player_loop::PlayerLoop;

//See new() to update version
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub portal:Portal,
	pub game:Game,

	tick:usize,

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String,
	viewport: Viewport,
}

//state loops definition
#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Render = 2,
	Exit = 3,
}

impl MainLoop {
	pub fn new(start_time:Time, terminal:DefaultTerminal) -> MainLoop {


		MainLoop {
			state: GameStates::Init,
			start:start_time.clone(),
			portal:Portal::new(),
			game:Game::new(),
			viewport: Viewport::new(80, 20, std::f32::consts::PI / 3.0),
			tick:0,

			//Set game version here
			logger: Logger::new(start_time, "0.4.0".to_string()),
			_output:String::new(),
			terminal:terminal,
		}
	}

	/// The main game loop. A plain synchronous loop now: set up once, then
	/// dispatch the current state forever — input, update, render, repeat.
	/// `handle_events` blocks on a keypress, which paces the loop. No async
	/// runtime, no recursion, no per-tick heap-allocated futures.
	pub fn run_game(&mut self) {
		self.logger.log("Initializing...");
		self.game.init(&mut self.logger);
		self.state = GameStates::Render;

		loop {
			match self.state {
				GameStates::Run    => self.run(),
				GameStates::Render => self.render(),
				GameStates::Exit   => self.exit(), // exits the process
				GameStates::Init   => self.state = GameStates::Render,
			}
		}
	}

	//running section of the main loop
	fn run(&mut self) {
		//event key which sends signals for game state and player movement
		let (new_state, player_input) =
			handle_events(&mut self.terminal, &mut self.logger);

		if new_state == GameStates::Exit {
			self.state = GameStates::Exit;
			return;
		}

		let (mut art, mut prompt, mut stats):(String,String,String) =
			(String::new(), String::new(), String::new());

		PlayerLoop::player_move(
			&mut self.game.player,
			player_input,
			&self.game.field,
			&mut self.logger,
		);

		self.game.field.set_entity(self.game.player.player.clone());
		self.game.update(&mut art, &mut prompt, &mut stats, self.tick, &mut self.logger);

		self.portal.set_portal(art, prompt, stats);

		self.tick += 1;
		self.logger.log(&format!("Tick: {}", self.tick));

		self.state = GameStates::Render;
	}

	fn render(&mut self) {
		// Get terminal size. Querying it can fail (e.g. a detached or non-tty
		// terminal); fall back to a conventional 80x24 rather than crashing.
		let (w, h) = match self.terminal.size() {
			Ok(size) => (size.width, size.height),
			Err(e) => {
				self.logger.log(&format!("Could not read terminal size: {e}; using 80x24"));
				(80, 24)
			}
		};

		self.logger.log(&format!("Size:{}x{}", w, h));

		// Update viewport dimensions
		self.viewport = Viewport::new(
			(w as usize).saturating_sub(2),
			(h as usize).saturating_sub(10),
			std::f32::consts::PI / 3.0
		);

		let player_pos = self.game.player.player.get_position();
		// Map the player's heading (0=up, 90=right, 180=down, -90=left) into the
		// ray caster's angle convention, where 0 rad points along +x and +y is
		// "down" on screen. Facing "up" is therefore -90 degrees of ray angle.
		let angle = (self.game.player.heading.0 as f32 - 90.0) * std::f32::consts::PI / 180.0;

		// Get walls from field entities (simplified: all entities are obstacles for now)
		let walls: Vec<(i16, i16)> = self
			.game
			.field
			.entities
			.values()
			.map(|e| (e.x, e.y))
			.filter(|&pos| pos != (player_pos.0, player_pos.1))
			.collect();

		// Render the viewport
		let view_text = self.viewport.render_raycasted(
			player_pos.0 as f32,
			player_pos.1 as f32,
			angle,
			&walls,
		);

		render(&mut self.terminal,
			&self.logger,
			&self.game.field,
			&view_text,
			player_pos,
			&self.portal,
		);

		self.state = GameStates::Run;
	}

	pub fn exit(&mut self) {
		// Save and quit. No async runtime to coordinate with anymore. We're on
		// the way out, so a failed flush/clear shouldn't panic over the exit.
		let _ = self.terminal.flush();
		let _ = self.terminal.clear();
		println!("Saving log...");
		self.logger.save_log();
		println!("Saved log.");
		println!("Exited.");
		std::process::exit(0x0);
	}
}
