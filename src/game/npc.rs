//! The stationary NPC — a being that stands in one place and, when spoken to,
//! gives the player a lens. It carries what the gaze and the dialogue surface:
//! a name and stats (for the Stats area), glyph art (for the overlay's view), and
//! the words it speaks. Its gift is a single [`Item`], taken once.
//!
//! The text here is inline for now; like flora it should move to a data file
//! (a `.being`/`.npc`) so the story lives at the file boundary, not in code.

use crate::game::entity::EntityId;
use crate::game::item::Item;

/// A stationary being: where it stands is the field's business (it's a solid
/// entity at a fixed cell), what it *is* lives here — and the one thing it has to
/// give, held in `lens` until the first time it's spoken to.
pub struct Npc {
	pub id: EntityId,
	pub glyph: char,
	pub name: String,
	/// Glyph art shown in the overlay's view when the gaze lands on it.
	pub art: String,
	/// The line it speaks when the player talks to it while it still holds the lens.
	pub words: String,
	/// What it says once the lens is already given — so it doesn't keep offering a
	/// gift it no longer holds.
	pub parting: String,
	/// The read-out the Stats area shows while you face it.
	pub stats: String,
	/// Its gift — taken once, on the first talk (see [`Game::talk`](crate::game::Game::talk)).
	pub lens: Option<Item>,
}

impl Npc {
	/// The Lenskeeper: the one stranger here, who gives a lens — not a weapon. The
	/// `lens_id` is minted alongside its own so the gift is a real, carryable item.
	pub fn lenskeeper(id: EntityId, lens_id: EntityId) -> Self {
		Npc {
			id,
			glyph: 'Ω',
			name: "the Lenskeeper".to_owned(),
			art: " ___\n(o o)\n/|=|\\\n  |  ".to_owned(),
			words: "Take this lens. Through it, the grey world shows its colours.".to_owned(),
			parting: "The lens is yours now. Go — find where the grey gives way.".to_owned(),
			stats: "the Lenskeeper\nHP 12   ATK 0".to_owned(),
			lens: Some(Item { id: lens_id, glyph: 'o', name: "a glass lens".to_owned() }),
		}
	}
}
