pub struct Closure {
	neighbors:Vec<i128>	
}

pub trait Closure {
	fn is_open() -> bool;
	pub fn get() -> Closure;
	pub fn make() -> Closure;
	
	//probably does everything
	fn get_nn() -> Vec<i128> {
		return neighbors
	}
}
