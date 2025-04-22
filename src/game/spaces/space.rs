use crate::utils::logger::Logger;

pub trait Space {
	fn new() -> Self;

	fn init(self, logger:&mut Logger) -> Self;
	fn update(self, _tick:i64) -> Self;
	fn save(self);
}