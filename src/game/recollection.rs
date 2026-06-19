//! The watcher's *memory* of what it has seen — vision accumulated over time.
//!
//! [`crate::game::vision::can_see`] answers "is she visible right now?" This
//! answers the gentler question the map actually needs: "where do I last
//! remember her, and how clearly?"
//!
//! A being you can see is known exactly — full clarity, at her true position.
//! The instant she slips from view she does not vanish; she becomes a *place*
//! you remember, frozen where you last saw her, and that memory **fades** tick
//! by tick until it is gone. The glyph on the map stops being her body and
//! becomes the last thing you knew about her.

use std::collections::{HashMap, HashSet};

/// One remembered sighting: a glyph at the place it was last seen, and how clear
/// that memory still is. `clarity` is `1.0` the instant it is seen and fades
/// toward `0.0` while it goes unseen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sighting {
	pub glyph: char,
	pub x: i16,
	pub y: i16,
	pub clarity: f32,
}

/// What the watcher remembers of the beings it has seen, keyed by being id.
pub struct Recollection {
	sightings: HashMap<String, Sighting>,
	/// Clarity lost per tick that a remembered being goes unseen.
	fade: f32,
}


//I need to read the ancient texts for this
impl Recollection {
	/// `fade` is how much clarity (out of `1.0`) is lost each tick a being goes
	/// unseen — e.g. `0.1` forgets a still-unseen being over ten ticks.
	pub fn new(fade: f32) -> Self {
		Self { sightings: HashMap::new(), fade }
	}

	/// Record a being seen *right now*: refreshed to full clarity at her true
	/// position. While you keep glimpsing her, the map tracks her exactly.
	pub fn glimpse(&mut self, id: &str, glyph: char, x: i16, y: i16) {
		self.sightings
			.insert(id.to_string(), Sighting { glyph, x, y, clarity: 1.0 });
	}

	/// Let one tick of forgetting pass. Every remembered being whose id is *not*
	/// in `seen_now` fades by `fade`; any memory that reaches zero clarity is
	/// forgotten entirely — she is no longer anywhere you know.
	///
	/// Call once per tick, *after* [`glimpse`](Self::glimpse)-ing everything
	/// visible this tick. `seen_now` must hold exactly those glimpsed ids, so a
	/// being still in view is left untouched at full clarity.
	pub fn fade_unseen(&mut self, seen_now: &HashSet<String>) {
		let fade = self.fade;
		self.sightings.retain(|id, s| {
			if seen_now.contains(id) {
				return true; // still seen — left at the clarity glimpse just set
			}
			s.clarity -= fade;
			s.clarity > 0.0
		});
	}

	/// What the watcher currently remembers, for the map to paint. Order is
	/// unspecified (backed by a hash map).
	pub fn recalled(&self) -> impl Iterator<Item = (&str, &Sighting)> {
		self.sightings.iter().map(|(id, s)| (id.as_str(), s))
	}

	/// The remembered sighting of one being, if any survives in memory.
	pub fn recall(&self, id: &str) -> Option<&Sighting> {
		self.sightings.get(id)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn seen(ids: &[&str]) -> HashSet<String> {
		ids.iter().map(|s| s.to_string()).collect()
	}

	fn clarity(r: &Recollection, id: &str) -> f32 {
		r.recall(id).expect("expected a surviving memory").clarity
	}

	#[test]
	fn a_glimpse_is_remembered_exactly() {
		let mut r = Recollection::new(0.25);
		r.glimpse("ooloonoo", 'F', 10, 10);

		let s = r.recall("ooloonoo").expect("just glimpsed her");
		assert_eq!((s.glyph, s.x, s.y), ('F', 10, 10));
		assert_eq!(s.clarity, 1.0);
	}

	#[test]
	fn while_seen_she_stays_perfectly_clear() {
		let mut r = Recollection::new(0.25);
		r.glimpse("ooloonoo", 'F', 10, 10);
		// She is among the seen this tick, so forgetting passes her by.
		r.fade_unseen(&seen(&["ooloonoo"]));
		assert_eq!(clarity(&r, "ooloonoo"), 1.0);
	}

	#[test]
	fn unseen_she_freezes_in_place_and_fades() {
		let mut r = Recollection::new(0.25);
		r.glimpse("ooloonoo", 'F', 10, 10);

		// Out of view now: the memory holds its position but loses clarity.
		r.fade_unseen(&seen(&[]));
		let s = r.recall("ooloonoo").expect("still faintly remembered");
		assert_eq!((s.x, s.y), (10, 10), "the place she was is what's remembered");
		assert!((s.clarity - 0.75).abs() < 1e-6, "clarity fell by one fade step");
	}

	#[test]
	fn fully_faded_is_forgotten() {
		let mut r = Recollection::new(0.5);
		r.glimpse("wisp", 'w', 3, 4);
		r.fade_unseen(&seen(&[])); // 1.0 -> 0.5
		assert!(r.recall("wisp").is_some());
		r.fade_unseen(&seen(&[])); // 0.5 -> 0.0, gone
		assert!(r.recall("wisp").is_none(), "a memory at zero clarity is no longer anywhere you know");
	}

	#[test]
	fn seeing_her_again_refreshes_and_follows_her() {
		let mut r = Recollection::new(0.25);
		r.glimpse("ooloonoo", 'F', 1, 1);
		r.fade_unseen(&seen(&[])); // faded to 0.75 at (1,1)

		// Caught sight of her again, somewhere new: full clarity, new place.
		r.glimpse("ooloonoo", 'F', 2, 2);
		let s = r.recall("ooloonoo").expect("seen again");
		assert_eq!((s.x, s.y), (2, 2), "the live sighting moves with her");
		assert_eq!(s.clarity, 1.0, "seeing her again makes the memory whole");
	}
}
