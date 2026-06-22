pub mod ui;
pub mod minimap;
pub mod portal;
pub mod viewport;

pub mod screen;
pub mod light;

use crate::{game::spaces::field::Field, utils::logger::Logger};
pub use viewport::Viewport;
use portal::Portal;

pub fn render(terminal: &mut ratatui::DefaultTerminal, log:&Logger, field:&Field, viewport_text:&String, player_pos:(i16, i16), portal:&Portal) {
	//thank you ratatui for TUI-ing so I could TUI something cool with the robots.
	let _ = terminal.draw(|frame| ui::draw_(frame, viewport_text, field, log, player_pos, portal) );

}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	let _ = terminal.clear();
}

