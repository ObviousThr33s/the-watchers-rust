



use crate::utils::logger;

use super::screen::Screen;

pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub art:String,
	pub prompt:String,
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), art:"none".to_owned(), prompt:"none".to_owned()}
	}
	
	pub fn set_portal(&mut self, art:String, prompt:String) {
		self.art = art;
		self.prompt = prompt;
	}

	pub fn build_screen(&mut self, width:i64) {
		self.screen.x = width;
		
		let art_lines = self.art.lines().count();
		let prompt_lines = self.prompt.lines().count();
		let max_lines = if art_lines > prompt_lines {art_lines} else {prompt_lines};
		

		let mut i = 0;

		let mut scr = String::new();

		loop {
			if i >= max_lines {
				break;
			}
			let art_line = self.art.lines().rev().nth(i).unwrap_or("");
			let prompt_line = self.prompt.lines().rev().nth(i).unwrap_or(".\n");
		
			let white_space = self.screen.x - ((art_line.len() + prompt_line.len()) as i64);
			let mut white_space_string:String = String::new();
			for _n in 0..white_space{
				white_space_string.push(' ');
			}

			let art_line = if let Some(pos) = art_line.find('\n') {
				&art_line[..pos]
			} else {
				art_line
			};

			let lin = format!("{}{}{}", art_line, white_space_string, prompt_line);
			
			logger::Logger::save_log_sp("./res/logs/", "screen", format!("{}", lin));
			
			scr.push_str(&lin.chars().rev().collect::<String>());
			
			i += 1;
		}

		self.screen.from_string(scr);

	}
}