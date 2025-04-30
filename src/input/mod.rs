use std::io::Stdout;

use ratatui::{
	crossterm::{self, event::KeyEventKind},  prelude::CrosstermBackend, Terminal
};

use crossterm::event::{self, Event, KeyCode};

use crate::{loops::main_loop::GameStates, utils::logger::Logger};


#[derive(PartialEq)]
pub enum PlayerMove {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	NONE
}

#[allow(unused_mut)]
pub async fn handle_events(terminal:&mut Terminal<CrosstermBackend<Stdout>>, mut logger:&mut Logger) -> (GameStates, PlayerMove) {
	let mut gs:GameStates = GameStates::Run;
	let mut mv:PlayerMove = PlayerMove::NONE;

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
			}
			KeyCode::Char('a') => {
				logger.log("a pressed");
				mv = PlayerMove::LEFT;
			}
			KeyCode::Char('s') => {
				logger.log("s pressed");
				mv = PlayerMove::DOWN;
			}
			KeyCode::Char('d') => {
				logger.log("d pressed");
				mv = PlayerMove::RIGHT;
			}
			KeyCode::Char(' ') => {
				let _ = terminal.autoresize();
				logger.log("space pressed and auto resize");
			}

			_ => {mv = PlayerMove::NONE} // Handle all other KeyCode variants
		},
		
		

		// handle other events
		Ok(Event::Resize(_,_)) => {
			logger.log("Resizing terminal");
			let _ = terminal.autoresize();
		}

		_ => { mv = PlayerMove::NONE }
	}

	(gs, mv)
}