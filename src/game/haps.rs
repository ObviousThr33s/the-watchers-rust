use crate::game::entity::EntityId;

/// A queued game action. Events are *data*: pushed during a tick, then drained
/// and applied in a deliberate phase, so "what should happen" stays separate from
/// "when it happens." Per the engine wards (see `CLAUDE.md`): a payload carries
/// values and [`EntityId`]s only — never a pointer, reference, lifetime, or
/// `String`. That keeps an `Event` `Copy`, heap-free, and trivially queueable.
///
/// To add a new kind of event: add a variant carrying everything the handler
/// needs (values and ids, nothing borrowed), then handle it where the queue is
/// drained.
///
/// `#[repr(C, u8)]` pins a defined, C-stable layout for the tagged union. Nothing
/// outside Rust reads it today, but the ward asks for flat, C-compatible payloads,
/// and the attribute costs nothing while keeping an exhaustive `match`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

//I don't know what this does and I am not sure you do
//#[repr(C, u8)]
pub enum Event {
	/// Spawn the sekaikan (Ooloonoo) at a position, addressed by its [`EntityId`].
	/// The readable name is *not* here — it lives in the being's data (component
	/// storage), looked up at mutation time. The event moves the fact, not the text.
	SpawnSekaikan { x: i16, y: i16, id: EntityId },
	/// Sync every game-piece into the field (and, later, run its behavior).
	AdvanceWatchers,
	/// Remove any game-piece whose health has dropped to zero.
	ReapDead,
}

/// A fixed-capacity ring buffer of [`Event`]s — the engine's event bus.
///
/// The queue is allocated once and never grows: `CAP` slots, zero heap traffic in
/// the hot loop (ward 2). Events come back first-in-first-out via [`pop`](Self::pop);
/// an overflowing [`push`](Self::push) returns the event as `Err` rather than
/// growing the buffer, so saturation is explicit and observable. Draw order is a
/// separate concern that lives on [`Priority`](crate::game::entity::Priority) — the
/// queue itself is pure arrival order.
///
/// `time` in [`Game`](crate::game::Game) is one of these, sized to the default.
pub struct Haps<const CAP: usize = 256> {
	/// The backing store. `Option` lets the array initialise to empty slots
	/// without an `unsafe` `MaybeUninit` and without a sham "Nil" event polluting
	/// the vocabulary; a live slot is `Some`, a free one `None`.
	slots: [Option<Event>; CAP],
	/// Index of the oldest queued event — the next one [`pop`](Self::pop) returns.
	head: usize,
	/// Index the next [`push`](Self::push) writes to.
	tail: usize,
	/// How many slots are live, so head/tail wrap without aliasing.
	len: usize,
}

impl<const CAP: usize> Haps<CAP> {
	/// An empty queue. `const` so it can be built in a `const` context.
	pub const fn new() -> Self {
		Self { slots: [None; CAP], head: 0, tail: 0, len: 0 }
	}

	/// Queue an event at the tail. Returns `Err(event)` if the ring is already
	/// full — the caller decides what to do with the overflow rather than the
	/// buffer silently growing (ward 2).
	pub fn push(&mut self, event: Event) -> Result<(), Event> {
		if self.len == CAP {
			return Err(event);
		}
		match self.slots.get_mut(self.tail) {
			Some(slot) => *slot = Some(event),
			None => return Err(event), // unreachable: tail is kept < CAP by the wrap
		}
		self.tail = (self.tail + 1) % CAP;
		self.len += 1;
		Ok(())
	}

	/// Take the oldest queued event, or `None` if the queue is empty (FIFO).
	pub fn pop(&mut self) -> Option<Event> {
		if self.len == 0 {
			return None;
		}
		let event = self.slots.get_mut(self.head).and_then(Option::take);
		self.head = (self.head + 1) % CAP;
		self.len -= 1;
		event
	}

	/// How many events are currently queued.
	pub fn len(&self) -> usize {
		self.len
	}

	/// Whether the queue holds no events.
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// Whether the queue is at capacity — the next [`push`](Self::push) will `Err`.
	pub fn is_full(&self) -> bool {
		self.len == CAP
	}
}

impl<const CAP: usize> Default for Haps<CAP> {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn spawn(id: EntityId) -> Event {
		Event::SpawnSekaikan { x: 0, y: 0, id }
	}

	#[test]
	fn pops_events_in_the_order_they_arrived() {
		let mut haps = Haps::<4>::new();
		haps.push(Event::AdvanceWatchers).unwrap();
		haps.push(Event::ReapDead).unwrap();
		haps.push(spawn(7)).unwrap();

		assert_eq!(haps.pop(), Some(Event::AdvanceWatchers));
		assert_eq!(haps.pop(), Some(Event::ReapDead));
		assert_eq!(haps.pop(), Some(spawn(7)));
		assert_eq!(haps.pop(), None, "draining an empty queue yields None");
	}

	#[test]
	fn a_full_ring_rejects_the_overflow_instead_of_growing() {
		let mut haps = Haps::<2>::new();
		haps.push(Event::AdvanceWatchers).unwrap();
		haps.push(Event::ReapDead).unwrap();
		assert!(haps.is_full());

		// The third push has nowhere to go: it comes straight back, untouched.
		assert_eq!(haps.push(spawn(1)), Err(spawn(1)));
		assert_eq!(haps.len(), 2, "a rejected push must not grow the ring");
	}

	#[test]
	fn the_ring_wraps_around_its_capacity() {
		let mut haps = Haps::<2>::new();
		haps.push(Event::AdvanceWatchers).unwrap();
		haps.push(Event::ReapDead).unwrap();
		assert_eq!(haps.pop(), Some(Event::AdvanceWatchers)); // frees the first slot

		// The tail wraps into the freed slot; arrival order is still preserved.
		haps.push(spawn(9)).unwrap();
		assert_eq!(haps.pop(), Some(Event::ReapDead));
		assert_eq!(haps.pop(), Some(spawn(9)));
		assert!(haps.is_empty());
	}
}
