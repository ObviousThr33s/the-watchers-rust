//! Input: reading keyboard events and turning them into a [`PlayerMove`] plus a
//! state signal for the loop. The one place key bindings live — WASD to move,
//! `q` to quit, space to force a resize.

use std::io::Stdout;
use ratatui::{
	crossterm::{self, event::KeyEventKind},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

use crate::{loops::main_loop::GameStates, utils::logger::Logger};


/// A directional intent read from the keyboard, or `NONE` for anything that does
/// not move the player.
#[derive(PartialEq, Debug)]
pub enum PlayerMove {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	/// Set a carried item down on the cell ahead.
	DROP,
	/// Talk to whatever the player faces.
	TALK,
	/// Enter or leave inspect mode (focus the Stats / Inventory read-outs).
	SCROLL,
	/// Move the inspect selection (arrow keys), or switch which box is focused.
	NavUp,
	NavDown,
	NavLeft,
	NavRight,
	NONE
}
//Controller support probably.
/// Block on one input event and translate it: returns the next [`GameStates`],
/// the [`PlayerMove`] it maps to, and whether the frame needs a redraw. Most
/// events don't warrant a repaint — only a move, a resize, or an explicit
/// refresh — which is what keeps the screen from strobing on key repeats.
#[allow(unused_mut)]
pub fn handle_events(terminal:&mut Terminal<CrosstermBackend<Stdout>>, mut logger:&mut Logger) -> (GameStates, PlayerMove, bool) {
	let mut gs:GameStates = GameStates::Run;
	let mut mv:PlayerMove = PlayerMove::NONE;
	// Whether this event warrants a repaint. Most events don't — only a move,
	// a resize, or an explicit refresh changes what's on screen. Leaving this
	// `false` for key releases/repeats and unmapped keys is what stops the
	// strobe (see the loop's use of it).
	let mut redraw = false;

	//sets and sends signals from keyboard to game system
	match event::read() {
		Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
			KeyCode::Char('q') => {
				logger.log("Exiting...");
				gs = GameStates::Exit;
			},
			// handle other key events
			KeyCode::Char('w') => {
				logger.log("w pressed");
				mv = PlayerMove::UP;
				redraw = true;
			}
			KeyCode::Char('a') => {
				logger.log("a pressed");
				mv = PlayerMove::LEFT;
				redraw = true;
			}
			KeyCode::Char('s') => {
				logger.log("s pressed");
				mv = PlayerMove::DOWN;
				redraw = true;
			}
			KeyCode::Char('d') => {
				logger.log("d pressed");
				mv = PlayerMove::RIGHT;
				redraw = true;
			}
			KeyCode::Char('r') => {
				logger.log("r pressed — drop");
				mv = PlayerMove::DROP;
				redraw = true;
			}
			KeyCode::Char('e') => {
				logger.log("e pressed — talk");
				mv = PlayerMove::TALK;
				redraw = true;
			}
			KeyCode::Char('v') | KeyCode::Char('V') => {
				logger.log("v pressed — inspect");
				mv = PlayerMove::SCROLL;
				redraw = true;
			}
			// Arrow keys drive the inspect selection (and are inert otherwise).
			KeyCode::Up => { mv = PlayerMove::NavUp; redraw = true; }
			KeyCode::Down => { mv = PlayerMove::NavDown; redraw = true; }
			KeyCode::Left => { mv = PlayerMove::NavLeft; redraw = true; }
			KeyCode::Right => { mv = PlayerMove::NavRight; redraw = true; }
			KeyCode::Char(' ') => {
				let _ = terminal.autoresize();
				logger.log("space pressed and auto resize");
				redraw = true;
			}

			_ => {mv = PlayerMove::NONE} // Handle all other KeyCode variants
		},



		// handle other events
		Ok(Event::Resize(_,_)) => {
			logger.log("Resizing terminal");
			let _ = terminal.autoresize();
			redraw = true;
		}

		_ => { mv = PlayerMove::NONE }
	}

	(gs, mv, redraw)
}