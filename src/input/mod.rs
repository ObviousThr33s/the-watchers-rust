use std::io::Stdout;

use ratatui::{
	crossterm::{self, event::KeyEventKind},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

pub fn handle_events(terminal:&mut Terminal<CrosstermBackend<Stdout>>) -> std::io::Result<bool> {
	match event::read()? {
		Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
			KeyCode::Char('q') => return Ok(true),
			// handle other key events
			_ => {} // Handle all other KeyCode variants
		},
		
		// handle other events
		Event::Resize(_,_) => {
			terminal.autoresize()?;
		}

		_ => ()
	}
	Ok(false)
}