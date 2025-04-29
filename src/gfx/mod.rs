pub mod ui;
pub mod mipmap;
pub mod portal;
pub mod charset;

mod screen;



use portal::Portal;

use crate::{game::group::Group, utils::logger::Logger};

pub fn render(terminal: &mut ratatui::DefaultTerminal, log:Logger, entity:Group, portal:Portal) {
	
	let _ = terminal.draw(|frame| ui::draw_(frame, portal, entity, log) );
	
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

