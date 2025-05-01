pub mod ui;
pub mod mipmap;
pub mod portal;
pub mod charset;

mod screen;

use screen::Screen;

use crate::{game::group::Group, utils::logger::Logger};

//render hooks

pub fn render(terminal: &mut ratatui::DefaultTerminal, log:Logger, entity:Group, screen:String) {
	
	let _ = terminal.draw(|frame| ui::draw_(frame, screen, entity, log) );
	
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

