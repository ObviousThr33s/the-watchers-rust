/// Portal stores metadata for rendering prompts and art
/// The actual screen rendering is handled by the viewport and render system
#[derive(Clone)]
pub struct Portal {
	pub art: String,
	pub prompt: String,
}

impl Portal {
	pub fn new() -> Self {
		Self {
			art: "none".to_owned(),
			prompt: "none".to_owned(),
		}
	}

	pub fn set_portal(&mut self, art: String, prompt: String) {
		self.art = art;
		self.prompt = prompt;
	}
}