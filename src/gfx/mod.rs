pub mod ui;
pub mod mipmap;

pub mod charset;

mod screen;



use crate::{game::group::Group, utils::logger::Logger};

pub fn render(terminal: &mut ratatui::DefaultTerminal, log:Logger, entity:Group) {
	
	let _ = terminal.draw(|frame| ui::draw_(frame, entity, log) );
	
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

