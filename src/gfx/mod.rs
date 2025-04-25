pub mod ui;
pub mod render;
mod screen;

use crate::{game::entity::Entity, utils::logger::Logger};

pub async fn render(terminal: &mut ratatui::DefaultTerminal, log:&mut Logger, entities:&mut Vec<Entity>) {
	let _ = terminal.draw(|frame| ui::draw_(frame, log.clone(), &mut entities.clone()));
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

