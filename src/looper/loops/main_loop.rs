use ratatui::DefaultTerminal;

use crate::{gfx, input::handle_events, utils::logger::Logger};
pub(crate) struct MainLoop {
	logger:Logger,
	tick:i64,
	terminal:DefaultTerminal,
}

impl MainLoop {
	pub fn new(){
		MainLoop { 
			tick: 0, 
			logger: Logger::new("0.1.9".to_string()),
			terminal: DefaultTerminal::new(),
		}
	}
	pub async fn run(&mut self) {
		loop {
		self.tick += 1;
		self.logger.log("Running");
		
		gfx::render(&mut self.terminal, &mut self.logger).await;
		
		if handle_events(&mut self.terminal).unwrap_or(false) {
			}
		}
	}
}
