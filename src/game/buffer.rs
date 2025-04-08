enum BufferStates {
	Idle,
	Running,
	Stopped
}
pub struct BufferObject {
	//note: must be used in a FIFO structure
	BufferStates:BufferStates

}

impl BufferObject {
	pub fn input_state(){}	
}

impl  BufferObject {
	pub fn render_state(){}
}

impl BufferObject {
	pub fn event_state(){}
}
