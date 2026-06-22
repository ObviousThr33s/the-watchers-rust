use std::io::Stdout;
use ratatui::{
	crossterm::{self, event::KeyEventKind},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

use crate::{loops::main_loop::GameStates, utils::logger::Logger};


#[derive(PartialEq, Debug)]
pub enum PlayerMove {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	NONE
}
//Controller support probably.
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