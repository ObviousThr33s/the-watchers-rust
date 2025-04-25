use std::io::Stdout;

use ratatui::{
	crossterm::{self, event::KeyEventKind, terminal},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

use crate::{game::entity::Entity, utils::logger::Logger};


#[allow(unused_mut)]
pub fn handle_events(terminal:&mut Terminal<CrosstermBackend<Stdout>>, mut logger:&mut Logger, mut e: &mut Entity) -> Entity {
	
	match event::read() {
		Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
			KeyCode::Char('q') => {
				std::process::exit(0x0);
			},
			// handle other key events
			KeyCode::Char('w') => {
				logger.log("w pressed");
				e.set_position(e.x, e.y-1);
				
			}
			KeyCode::Char('a') => {
				logger.log("a pressed");
				e.set_position(e.x-1, e.y);
			}
			KeyCode::Char('s') => {
				logger.log("s pressed");
				e.set_position(e.x, e.y+1);
			}
			KeyCode::Char('d') => {
				logger.log("d pressed");
				e.set_position(e.x+1, e.y);
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
			terminal.autoresize();
		}

		_ => ()
	}

	e.clone()
}