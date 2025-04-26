pub mod ui;
pub mod mipmap;

pub mod charset;

mod screen;

use crate::{game::entity::Entity, utils::logger::Logger};

pub async fn render(terminal: &mut ratatui::DefaultTerminal, log:Logger, entities:&mut Vec<Entity>) -> (Vec<Entity>, Vec<(u16,u16)>){
	let mut v: Vec<(u16, u16)> = Vec::new();
	let _ = terminal.draw(|frame| { v = ui::draw_(frame, &mut entities.clone(), log); });
	
	(entities.to_vec(), v.clone())
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

