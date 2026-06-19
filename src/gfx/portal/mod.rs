/// Portal stores metadata for rendering prompts and art
/// The actual screen rendering is handled by the viewport and render system
//what are we going to do when we add portals in. maybe rename this to Porthole

#[derive(Clone)]
pub struct Portal {
	pub art: String,
	pub prompt: String,
	/// The seen entity's stat readout (name / HP / ATK), shown in the Stats
	/// panel. Empty when nothing is in view — gaze is what fills it.
	pub stats: String,
}

impl Portal {
	pub fn new() -> Self {
		Self {
			art: "none".to_owned(),
			prompt: "none".to_owned(),
			stats: String::new(),
		}
	}

	pub fn set_portal(&mut self, art: String, prompt: String, stats: String) {
		self.art = art;
		self.prompt = prompt;
		self.stats = stats;
	}
}