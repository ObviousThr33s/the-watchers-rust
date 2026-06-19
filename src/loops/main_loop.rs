
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
	pub game:Game,

	tick:usize,

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String,
	viewport: Viewport,
	portal: Portal,
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
	pub fn new(start_time:Time, terminal:DefaultTerminal, game:Game) -> MainLoop {
		MainLoop {
			game,
			start:start_time.clone(),
			
			state: GameStates::Init,
			
			viewport: Viewport::new(80, 20, std::f32::consts::PI / 3.0),
			tick:0,

			//Set game version here
			logger: Logger::new(start_time, "0.5.4".to_string()),
			_output:String::new(),
			terminal:terminal,
			portal: Portal::new(),
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
		/*
		PlayerLoop::player_move(
			&mut self.game.player,
			player_input,
			&self.game.field,
			&mut self.logger,
		);
		*/
		
		//self.game.update(self, self.tick, &mut self.logger);

		//self.portal.set_portal(art, prompt, stats);

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

		// Size the first-person viewport to the terminal, leaving room for the UI
		// chrome (borders + the bottom panels).
		self.viewport = Viewport::new(
			(w as usize).saturating_sub(2),
			(h as usize).saturating_sub(10),
			std::f32::consts::PI / 3.0,
		);

		// The player lives in the field now (id "Player"); read the position from
		// there. The fallback keeps render() honest if the player is ever absent.
		let player_pos = self
			.game
			.field
			.get_entity_by_id("Player")
			.map(|p| p.get_position())
			.unwrap_or((2, 2));

		// Heading is fixed "up" for now — turning/movement is the next pass. Map
		// "up" into the ray caster's convention (0 rad = +x, +y = down on screen),
		// so facing up is -90 degrees.
		let angle = -std::f32::consts::FRAC_PI_2;

		// Every entity other than the player is a solid wall for the ray caster.
		let walls: Vec<(i16, i16)> = self
			.game
			.field
			.entities
			.values()
			.map(|e| (e.x, e.y))
			.filter(|&pos| pos != player_pos)
			.collect();

		let view = self.viewport.render_raycasted(
			player_pos.0 as f32,
			player_pos.1 as f32,
			angle,
			&walls,
		);

		render(
			&mut self.terminal,
			&self.logger,
			&self.game.field,
			&view,
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
