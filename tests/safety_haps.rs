//! Absolute-safety tests for the `Haps` event ring — the bounded-bus contract.
//!
//! The ring is allocated once (engine ward 2, see `CLAUDE.md`). The promise this
//! crate guards: no sequence of pushes and pops can make it panic, grow, or
//! violate `len <= CAP`. Events stay pure, copyable data (ward 1).
//!
//! Integration tests (ch. 11.3): public API only, with shared fixtures pulled in
//! from `tests/common/mod.rs`. Unit tests for the ring's basic mechanics live
//! inline in `src/game/haps.rs`; these are the adversarial/invariant layer on top.

mod common;

use common::spawn;
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

/// The core invariant under a long, deterministic mix of pushes and pops: `len`
/// never exceeds `CAP`, and the ring always agrees with a simple FIFO model —
/// nothing is lost, duplicated, or reordered. Deterministic (a seeded xorshift)
/// so a failure reproduces exactly.
#[test]
fn interleaved_push_pop_stays_bounded_and_fifo() {
	const CAP: usize = 8;
	let mut ring: Haps<CAP> = Haps::new();
	let mut model: std::collections::VecDeque<u64> = std::collections::VecDeque::new();

	let mut seed: u64 = 0x1234_5678_9abc_def0;
	let mut next_id: u64 = 0;

	for _ in 0..100_000 {
		// xorshift: deterministic, no rand dependency in the safety net.
		seed ^= seed << 13;
		seed ^= seed >> 7;
		seed ^= seed << 17;

		if seed & 1 == 0 {
			let event = spawn(next_id);
			match ring.push(event) {
				Ok(()) => {
					assert!(model.len() < CAP, "push succeeded only because there was room");
					model.push_back(next_id);
				}
				Err(returned) => {
					assert_eq!(returned, event, "a rejected push hands the event straight back");
					assert_eq!(model.len(), CAP, "push rejected only because the ring was full");
				}
			}
			next_id += 1;
		} else {
			let expected = model.pop_front().map(spawn);
			assert_eq!(ring.pop(), expected, "the ring drains in first-in-first-out order");
		}

		assert!(ring.len() <= CAP, "len must never exceed capacity");
		assert_eq!(ring.len(), model.len(), "the ring and the model never diverge");
	}
}
