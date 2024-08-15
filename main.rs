pub mod utility;

use utility::utils::utils::{self as utils, Screen};

fn main(){
	let s = Screen::new(30,30);
	Screen::to_string(s);
	print!("Screen initialized.");
	
	let mut i: u32 = 70;

	let mut line: String = String::new();

	while i < 90 {
		i += 1;
		utils::render(i);
		
		std::io::stdin().read_line(&mut line).unwrap();
	}
	

}
