use crate::utils::input;

pub(crate) struct Event {
	id:i128
}

impl Event {
	pub fn get_input() -> bool{
		input::Input::get_input();
		false
	}
}
