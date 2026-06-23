//! The entity backbone: a fixed-capacity slab the field is built on.
//!
//! This is the "stack" half of the memory plan — reserved once at init and never
//! grown at runtime, so it never reallocates. An [`EntityId`] is a *direct index*
//! into it (ids climb densely from `0`), so every lookup is an index, not a hash.
//! Empty slots read `None`.

use crate::game::entity::{Entity, EntityId};

/// Worst-case entity count reserved up front. Allocated once in [`Roster::new`]
/// and never grown; an id past it is refused rather than triggering a realloc.
///
/// Provisional: ids only climb today (no slot reuse), so this must exceed every
/// id minted in a run. A free-list that recycles slots — and a generation on the
/// id, so a recycled slot can't answer to a stale handle — is the next organ, and
/// it's what turns this capacity into a true bound instead of a high-water mark.
const CAP: usize = 1 << 16;

/// A fixed-capacity slab of entities, addressed directly by [`EntityId`].
#[derive(Clone)]
pub struct Roster {
	slots: Vec<Option<Entity>>,
	/// How many slots are filled — kept as a running count so [`len`](Self::len)
	/// never has to scan.
	count: usize,
	/// One past the highest id ever placed, so iteration walks only the live
	/// prefix instead of the whole reserved slab.
	high: usize,
}

impl Roster {
	/// Reserve the whole backbone at once — the only allocation a roster ever
	/// makes. Every slot starts empty.
	pub fn new() -> Self {
		let mut slots = Vec::new();
		slots.resize_with(CAP, || None);
		Roster { slots, count: 0, high: 0 }
	}

	/// Place `entity` in the slot its id names. An id beyond the reserved capacity
	/// is **refused** (returns `false`) rather than growing the slab: overflow is
	/// an explicit, observable condition, never a silent realloc.
	pub fn insert(&mut self, entity: Entity) -> bool {
		let i = entity.id as usize;
		match self.slots.get_mut(i) {
			Some(slot) => {
				if slot.is_none() {
					self.count += 1;
				}
				*slot = Some(entity);
				self.high = self.high.max(i + 1);
				true
			}
			None => false,
		}
	}

	/// Take the entity out of `id`'s slot, if one is there.
	pub fn remove(&mut self, id: EntityId) -> Option<Entity> {
		let taken = self.slots.get_mut(id as usize).and_then(Option::take);
		if taken.is_some() {
			self.count -= 1;
		}
		taken
	}

	/// Borrow the entity at `id`, if present.
	#[inline]
	pub fn get(&self, id: EntityId) -> Option<&Entity> {
		self.slots.get(id as usize).and_then(Option::as_ref)
	}

	/// Mutably borrow the entity at `id`, if present.
	#[inline]
	pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
		self.slots.get_mut(id as usize).and_then(Option::as_mut)
	}

	/// Every entity currently held, in id order. Empty slots are skipped, and only
	/// the live prefix is walked, never the whole reserved slab.
	pub fn values(&self) -> impl Iterator<Item = &Entity> {
		self.slots.iter().take(self.high).flatten()
	}

	/// How many slots are filled.
	#[inline]
	pub fn len(&self) -> usize {
		self.count
	}

	/// Whether no slot is filled.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.count == 0
	}
}

impl Default for Roster {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::Priority;

	fn at(id: EntityId) -> Entity {
		Entity::new(0, 0, '#', id, Priority::LOW)
	}

	#[test]
	fn an_id_is_its_own_slot() {
		let mut r = Roster::new();
		assert!(r.insert(at(7)));
		assert_eq!(r.get(7).map(|e| e.id), Some(7), "the id indexes straight to its slot");
		assert!(r.get(8).is_none(), "an untouched slot reads empty");
	}

	#[test]
	fn remove_frees_the_slot_and_the_count() {
		let mut r = Roster::new();
		r.insert(at(3));
		assert_eq!(r.len(), 1);
		assert_eq!(r.remove(3).map(|e| e.id), Some(3));
		assert_eq!(r.len(), 0, "removing empties the slot");
		assert!(r.get(3).is_none());
	}

	#[test]
	fn an_id_past_capacity_is_refused_not_grown() {
		let mut r = Roster::new();
		assert!(!r.insert(at(CAP as u64)), "an id beyond the reserved slab is refused");
		assert!(r.is_empty(), "a refused insert places nothing");
	}
}
