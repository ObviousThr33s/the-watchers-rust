use std::io;
pub(crate) struct Input {}

impl Input {
	pub fn get_input() -> String{
		//get input
		let mut input = String::new();
		//handle errors
		match io::stdin().read_line(&mut input) {
			Ok(_n) => {
				//println!("{n} bytes read");
				println!("{input}");
			}Err(error) => println!("error: {error}"),
		}
		////

		input
	}
}