use crate::game::entity::Priority;

/// A queued game action. Events are *data*: pushed during a tick, then drained
/// and applied in priority order. This keeps "what should happen" separate from
/// "when it happens," and makes a tick's work easy to inspect or log.
///
/// To add a new kind of event: add a variant here carrying everything the
/// handler needs, then handle it in `Game::apply` (game/mod.rs).
pub enum Event {
	/// Spawn the fairy (Ooloonoo) at a position, with a name and id.
	SpawnFairy { x: i16, y: i16, name: String, id: String },
	/// Sync every game-piece into the field (and, later, run its behavior).
	AdvanceWatchers,
	/// Remove any game-piece whose health has dropped to zero.
	ReapDead,
}

/// A small, priority-ordered queue of [`Event`]s for a single game tick.
///
/// Unlike the old version, this actually defers work: events are collected,
/// then `drain_by_priority` hands them back highest-priority first so the
/// caller applies them in a deliberate order.
pub struct Haps {
	queue: Vec<(Priority, Event)>,
}

impl Haps {
	pub fn new() -> Self {
		Self { queue: Vec::new() }
	}

	/// Queue an event to run this tick at the given priority.
	pub fn push(&mut self, priority: Priority, event: Event) {
		self.queue.push((priority, event));
	}

	/// Take all queued events, highest priority first. The sort is stable, so
	/// events sharing a priority keep their insertion order.
	pub fn drain_by_priority(&mut self) -> Vec<Event> {
		self.queue.sort_by(|a, b| b.0.cmp(&a.0));
		self.queue.drain(..).map(|(_, event)| event).collect()
	}

	//We need blocks/chunks and major modal clears vs minor modal clears.
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::Priority;

	#[test]
	fn drains_highest_priority_first() {
		
		let mut haps = Haps::new();
		haps.push(Priority::LOW, Event::ReapDead);
		haps.push(Priority::HIG, Event::AdvanceWatchers);
		haps.push(Priority::MED, Event::SpawnFairy { x: 0, y: 0, name: "x".into(), id: "1".into() });

		let order = haps.drain_by_priority();

		assert!(matches!(order[0], Event::AdvanceWatchers));      // HIG
		assert!(matches!(order[1], Event::SpawnFairy { .. }));    // MED
		assert!(matches!(order[2], Event::ReapDead));             // LOW
	}

	#[test]
	fn ties_keep_insertion_order() {
		let mut haps = Haps::new();
		haps.push(Priority::MED, Event::AdvanceWatchers);
		haps.push(Priority::MED, Event::ReapDead);

		let order = haps.drain_by_priority();

		assert!(matches!(order[0], Event::AdvanceWatchers));
		assert!(matches!(order[1], Event::ReapDead));
	}
}
