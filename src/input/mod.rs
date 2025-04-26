use std::io::Stdout;

use ratatui::{
	crossterm::{self, event::KeyEventKind},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

use crate::{game::entity::Entity, looper::looper::GameStates, utils::logger::Logger};


#[allow(unused_mut)]
pub fn handle_events(terminal:&mut Terminal<CrosstermBackend<Stdout>>, mut logger:&mut Logger, mut e: &mut Vec<Entity>) -> GameStates {
	let mut gs:GameStates = GameStates::Run;

	match event::read() {
		Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
			KeyCode::Char('q') => {
				logger.log("Exiting...");
				gs = GameStates::Exit;
			},
			// handle other key events
			KeyCode::Char('w') => {
				logger.log("w pressed");
				e[0].move_up();
				
			}
			KeyCode::Char('a') => {
				logger.log("a pressed");
				e[0].move_left();
			}
			KeyCode::Char('s') => {
				logger.log("s pressed");
				e[0].move_down();
			}
			KeyCode::Char('d') => {
				logger.log("d pressed");
				e[0].move_right();
			}
			KeyCode::Char(' ') => {
				let _ = terminal.autoresize();
				logger.log("space pressed and auto resize");
			}

			_ => {} // Handle all other KeyCode variants
		},
		
		

		// handle other events
		Ok(Event::Resize(_,_)) => {
			logger.log("Resizing terminal");
			let _ = terminal.autoresize();
		}

		_ => ()
	}

	gs
}