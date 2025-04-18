pub mod ui;
pub mod lamp;
mod screen;
use std::io::Stdout;

use crate::utils::logger::Logger;

pub async fn render(terminal: &mut ratatui::DefaultTerminal, log:&mut Logger) {
	let _ = terminal.draw(|frame| ui::draw_(frame, log.clone()));
}

pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	terminal.clear().unwrap()
}

