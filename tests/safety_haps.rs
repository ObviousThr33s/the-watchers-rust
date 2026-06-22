//! Absolute-safety tests for the `Haps` event ring — the bounded-bus contract.
//!
//! The ring is allocated once (engine ward 2, see `CLAUDE.md`). The promise this
//! crate guards: no sequence of pushes and pops can make it panic, grow, or
//! violate `len <= CAP`. Events stay pure, copyable data (ward 1).
//!
//! Integration tests (ch. 11.3): public API only, with shared fixtures pulled in
//! from `tests/common/mod.rs`. Unit tests for the ring's basic mechanics live
//! inline in `src/game/haps.rs`; these are the adversarial/invariant layer on top.

use obelisk::game::haps::{Event, Haps};

/// Events are pure, copyable data (engine ward 1). This is a compile-time proof:
/// if `Event` ever grows a reference, lifetime, or heap field, it stops being
/// `Copy` and this crate fails to build.
#[test]
fn events_are_copy_pure_data() {
	fn requires_copy<T: Copy>() {}
	requires_copy::<Event>();
}

/// A zero-capacity ring is degenerate but must still be panic-free: it accepts
/// nothing and yields nothing, and never divides by its capacity.
#[test]
fn a_zero_capacity_ring_never_panics() {
	let mut ring: Haps<0> = Haps::new();
	assert!(ring.is_full());
	assert!(ring.is_empty());
	assert_eq!(ring.push(Event::ReapDead), Err(Event::ReapDead));
	assert_eq!(ring.pop(), None);
	assert_eq!(ring.len(), 0);
}

/// Popping an empty ring yields `None` indefinitely — no underflow, no panic.
#[test]
fn popping_an_empty_ring_yields_none_forever() {
	let mut ring: Haps<4> = Haps::new();
	for _ in 0..16 {
		assert_eq!(ring.pop(), None);
	}
}

/// Once full, every further push is rejected and the ring does not grow — the
/// bounded-allocation ward holds under sustained overflow.
#[test]
fn a_full_ring_rejects_overflow_without_growing() {
	let mut ring: Haps<3> = Haps::new();
	for _ in 0..3 {
		ring.push(Event::AdvanceWatchers).expect("fits");
	}
	for _ in 0..1_000 {
		assert_eq!(ring.push(Event::ReapDead), Err(Event::ReapDead));
		assert_eq!(ring.len(), 3, "a rejected push must never grow the ring");
	}
}

// PARKED — the identity-FIFO property test once lived here. It drove the ring with
// `common::spawn(id)`, minting a *distinguishable* event per id so a VecDeque model
// could prove nothing was reordered, lost, or duplicated. The SpawnSekaikan retreat
// removed the only value-bearing `Event` variant, so the remaining unit variants
// (`AdvanceWatchers`/`ReapDead`) can't tell one queued event from another — the test
// would degrade to checking counts, not order. Restore it (and `common::spawn`) when
// a value-carrying Event returns; the original is in git history.
