use ratatui::DefaultTerminal;

use crate::{gfx, input::handle_events, utils::logger::Logger};
pub(crate) struct MainLoop {
	logger:Logger,
	tick:i64,
	terminal:DefaultTerminal,
}

impl MainLoop {
	
}
