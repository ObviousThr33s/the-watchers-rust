//! The main loop: the game's heartbeat. [`MainLoop`] owns the terminal, the
//! [`Game`], and the per-frame state, then runs a plain synchronous state machine
//! ([`GameStates`]) — input, update, render — paced by a blocking key read. No
//! async runtime, no recursion, no per-tick heap-allocated futures.

use ratatui::DefaultTerminal;
use crate::game::Game;
use crate::game::entity::PLAYER;
use crate::gfx::voxel::Voxel;
use crate::gfx::portal::Portal;
use crate::gfx::render;
use crate::input::{handle_events, PlayerMove};
use crate::utils::{logger::Logger, time::Time};

/// Map a directional key to the player's response: a grid step `(dx, dy)`, the
/// ray-caster angle to face, and the glyph that shows that facing on the Map.
/// Any non-movement key (`NONE`) leaves the player as they are.
fn facing_of(input: &PlayerMove) -> Option<(i16, i16, f32, char)> {
	use std::f32::consts::{FRAC_PI_2, PI};
	match input {
		PlayerMove::UP    => Some((0, -1, -FRAC_PI_2, '^')),
		PlayerMove::DOWN  => Some((0,  1,  FRAC_PI_2, 'v')),
		PlayerMove::LEFT  => Some((-1, 0,  PI,        '<')),
		PlayerMove::RIGHT => Some((1,  0,  0.0,       '>')),
		PlayerMove::NONE  => None,
	}
}

//See new() to update version
/// Everything one running session needs: the run clock and logger, the [`Game`]
/// world, the terminal, the first-person [`Voxel`] view, the [`Portal`] reveal
/// state, and the angle the player currently faces.
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub game:Game,

	tick:usize,

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String,
	voxel: Voxel,
	portal: Portal,
	/// The ray-caster angle the player is currently facing (updated on each
	/// directional key). Starts facing "up" to match the player's `^` glyph.
	facing: f32,
}

//state loops definition
/// The loop's state machine. Each pass dispatches on the current state and sets
/// the next one.
#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Render = 2,
	Exit = 3,
}

impl MainLoop {
	/// Build a session around an initialized terminal and a fresh [`Game`].
	/// `version` is stamped into the logger and shown in the UI header.
	pub fn new(start_time:Time, terminal:DefaultTerminal, game:Game, version:String) -> MainLoop {
		MainLoop {
			game,
			start:start_time.clone(),
			
			state: GameStates::Init,
			
			voxel: Voxel::new(80, 20, std::f32::consts::PI / 3.0),
			tick:0,

			//Set game version here
			logger: Logger::new(start_time, version),
			_output:String::new(),
			terminal:terminal,
			portal: Portal::new(),
			facing: -std::f32::consts::FRAC_PI_2, // facing "up", matching '^'
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
		let (new_state, player_input, redraw) =
			handle_events(&mut self.terminal, &mut self.logger);

		if new_state == GameStates::Exit {
			self.state = GameStates::Exit;
			return;
		}

		// A directional key turns the player to face that way and tries to step
		// one cell; a wall blocks the step but the turn still happens. Movement is
		// grid-based for now — the view and Map follow because they read the
		// player's position straight from the field.
		if let Some((dx, dy, angle, glyph)) = facing_of(&player_input) {
			self.facing = angle;
			self.game.field.move_entity(PLAYER, dx, dy);
			if let Some(player) = self.game.field.get_entity_by_id_mut(PLAYER) {
				player.self_ = glyph;
			}
		}

		// Only repaint when the event actually changed something. Windows delivers
		// key releases and repeats in bursts; redrawing (and ticking the log) on
		// every one of them is what made the screen strobe. An idle event drops
		// straight back to the blocking read instead.
		if redraw {
			self.tick += 1;
			self.logger.log(&format!("Tick: {}", self.tick));
			self.state = GameStates::Render;
		} else {
			self.state = GameStates::Run;
		}
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

		// Size the first-person voxel view to the terminal, leaving room for the UI
		// chrome (borders + the bottom panels).
		self.voxel = Voxel::new(
			(w as usize).saturating_sub(2),
			(h as usize).saturating_sub(10),
			std::f32::consts::PI / 3.0,
		);

		// The player lives in the field now (id PLAYER); read the position from
		// there. The fallback keeps render() honest if the player is ever absent.
		let player_pos = self
			.game
			.field
			.get_entity_by_id(PLAYER)
			.map(|p| p.get_position())
			.unwrap_or((2, 2));

		// Face the way the player last moved. The ray caster's convention: 0 rad
		// is +x, +y is down on screen, so "up" is -90 degrees (set in `new`, then
		// updated by `run` on each directional key).
		let angle = self.facing;

		// The first-person view is the ground itself now: march the heightmap into
		// relief from where the player stands and faces. Field entities (flora,
		// beings) drop out of this panel until the sprite pass composites them back
		// over the terrain — they still show on the top-down Map meanwhile.
		let view = self.voxel.render(
			player_pos.0 as f32,
			player_pos.1 as f32,
			angle,
			&self.game.ground,
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
