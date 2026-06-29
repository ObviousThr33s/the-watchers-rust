//! Finish line — "the end of the universe."
//!
//! METHOD.md: a branch is cut to cross one finish line — a single red test that
//! says what *done* means. This is that test for the liminal fade: the
//! `docs/lore/mnemosyne-and-lethe.md` seed, graduated in
//! `docs/ideologues/the-liminal-fade.md`.
//!
//! The end of the universe is the place where movement goes so liminally still
//! it never quite arrives: a remembered being held forever, its *place* frozen
//! while its *clarity* keeps fading toward a zero it never reaches. The world's
//! ordinary fade is linear — it reaches zero and forgets (see `recollection.rs`
//! :: `fully_faded_is_forgotten`). So this is red until
//! `Recollection::at_worlds_end` gives memory a fade that only *approaches* zero.

use std::collections::HashSet;

use obelisk::game::recollection::Recollection;

/// At the universe's end a memory is never wholly forgotten: its place holds
/// still, and though it fades from full it lingers forever at the threshold —
/// movement, but so liminal it never arrives at zero.
#[test]
fn at_the_universes_end_a_memory_lingers_still_and_never_arrives() {
	// A recollection at the end of the universe: forgetting that only ever
	// approaches zero, never completing.
	let mut horizon = Recollection::at_worlds_end();
	horizon.glimpse("oolooroo", 'F', 7, 7);

	let unseen = HashSet::new();

	// It begins to fade the instant it goes unseen — it is not frozen at full.
	horizon.fade_unseen(&unseen);
	let first_clarity = horizon
		.recall("oolooroo")
		.expect("a world's-end memory survives the first unseen tick")
		.clarity;
	assert!(first_clarity < 1.0, "it moves — the fade takes hold");

	// However long the universe runs, the place stays put and the memory is
	// never wholly forgotten: clarity approaches zero but never arrives.
	for tick in 0..100_000 {
		horizon.fade_unseen(&unseen);
		let s = horizon
			.recall("oolooroo")
			.expect("a world's-end memory is never wholly forgotten");

		assert_eq!((s.x, s.y), (7, 7), "the place is still — the fade never moves it");
		assert!(s.clarity > 0.0, "clarity approaches zero but never arrives");

		// Some way in, it is plainly still moving — liminal motion, not a freeze
		// that merely stopped at full.
		if tick == 64 {
			assert!(
				s.clarity < first_clarity,
				"it keeps fading — it has not simply stopped"
			);
		}
	}
}
