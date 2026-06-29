//! The fairy — a being that flits around the forest. It shows for a few beats,
//! slips out of sight for a few, and reappears a little way off around its home,
//! so it reads as a thing glimpsed between the trees, never fixed in place. Pure
//! mechanism: where it haunts is data (its `home`), the flicker is its nature.

use crate::game::entity::EntityId;

/// The glyph the fairy wears on the Map and in the field.
pub const FAIRY_GLYPH: char = '✦';

/// A flitting being: a home it haunts, and a beat counter that paces its coming
/// and going. It carries the [`EntityId`] it answers to so the world can lift it
/// off one cell and set it on another without ever confusing it for anything else.
pub struct Fairy {
	pub id: EntityId,
	home: (i16, i16),
	phase: u32,
}

impl Fairy {
	/// A fairy that haunts the cells around `home`, answering to `id`.
	pub fn new(id: EntityId, home: (i16, i16)) -> Self {
		Fairy { id, home, phase: 0 }
	}

	/// Advance one beat. Returns the cell the fairy haunts now, or `None` when it
	/// has slipped out of sight. It is seen for the first stretch of each cycle and
	/// gone for the rest; each appearance stands a little way off around its home,
	/// so across beats it wanders rather than blinking in one spot.
	pub fn flit(&mut self) -> Option<(i16, i16)> {
		const CYCLE: u32 = 6; // beats per come-and-go
		const SEEN: u32 = 3; // of those, how many it is visible

		self.phase = self.phase.wrapping_add(1);
		if self.phase % CYCLE >= SEEN {
			return None; // out of sight for this stretch
		}

		// Which appearance this is decides where it stands — a small wander box
		// around home, repeating but never the same two trips running.
		let trip = self.phase / CYCLE;
		let dx = (trip % 5) as i16 - 2; // -2..=2
		let dy = (trip / 5 % 5) as i16 - 2; // -2..=2
		Some((self.home.0 + dx, self.home.1 + dy))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;

	#[test]
	fn the_fairy_appears_and_disappears() {
		let mut f = Fairy::new(1, (10, 10));
		let beats: Vec<_> = (0..12).map(|_| f.flit()).collect();
		assert!(beats.iter().any(Option::is_some), "the fairy is seen on some beats");
		assert!(beats.iter().any(Option::is_none), "and gone on others");
	}

	#[test]
	fn it_wanders_around_its_home_not_a_fixed_cell() {
		let mut f = Fairy::new(1, (10, 10));
		let spots: HashSet<(i16, i16)> = (0..60).filter_map(|_| f.flit()).collect();
		assert!(spots.len() >= 2, "it doesn't haunt a single fixed cell");
		assert!(
			spots.iter().all(|&(x, y)| (x - 10).abs() <= 2 && (y - 10).abs() <= 2),
			"but it stays around its home",
		);
	}
}
