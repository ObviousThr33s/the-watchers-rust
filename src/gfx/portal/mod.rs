/// Portal stores metadata for rendering prompts and art
/// The actual screen rendering is handled by the viewport and render system
//what are we going to do when we add portals in. maybe rename this to Porthole

#[derive(Clone)]
pub struct Portal {
	/// The seen entity's ascii art, or `"none"` when nothing is in view.
	pub art: String,
	/// The line the seen entity surfaces, or `"none"` when nothing is in view.
	pub prompt: String,
	/// The seen entity's stat readout (name / HP / ATK), shown in the Stats
	/// panel. Empty when nothing is in view — gaze is what fills it.
	pub stats: String,
}

impl Portal {
	/// An empty portal — nothing seen yet (`art`/`prompt` read `"none"`, stats blank).
	pub fn new() -> Self {
		Self {
			art: "none".to_owned(),
			prompt: "none".to_owned(),
			stats: String::new(),
		}
	}

	/// Fill the portal from a freshly-seen entity: its art, the line it surfaces,
	/// and a stat read-out.
	pub fn set_portal(&mut self, art: String, prompt: String, stats: String) {
		self.art = art;
		self.prompt = prompt;
		self.stats = stats;
	}
}