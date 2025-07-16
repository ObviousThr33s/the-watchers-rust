pub mod ui;
pub mod minimap;
pub mod portal;
pub mod charset;

mod screen;

use crate::{game::spaces::field::Field, utils::logger::Logger};

//render hooks

pub async fn render(terminal: &mut ratatui::DefaultTerminal, log:&Logger, field:&Field, screen:&String) {
	
	let _ = terminal.draw(|frame| ui::draw_(frame, screen, field, log) );
	
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

