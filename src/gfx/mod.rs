pub mod ui;
pub mod mipmap;

pub mod charset;

mod screen;

use crate::{game::{entity::Entity, group::Group}, utils::logger::Logger};

pub async fn render(terminal: &mut ratatui::DefaultTerminal, log:Logger, entity:Group) {
	
	let _ = terminal.draw(|frame| ui::draw_(frame, entity, log) );
	
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

