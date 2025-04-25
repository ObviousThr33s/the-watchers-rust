
use core::time;

use ratatui::crossterm::event;

use crate::game::{entity::{self, Entity}};
pub(crate) struct MainLoop {
	tick:usize,
}

impl MainLoop {


	pub fn new() -> MainLoop {
		MainLoop { 
			tick: 0, 
		}
	}

	pub async fn main_loop(&mut self) {
		
	}
}
