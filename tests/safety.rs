//! Absolute-safety tests — the engine's hostile-input contract.
//!
//! These are not feature tests; they are the promise that no input can make the
//! engine *panic, overflow, or grow unbounded*. Two surfaces take untrusted
//! data:
//!
//! * `Being::parse` — `.being` files are content, authored by anyone in a public,
//!   collaborative repo. A malformed or adversarial file must always resolve to
//!   `Ok` or a typed `Err`, never a crash.
//! * `Haps` — the bounded event ring (engine ward 2). It is allocated once and
//!   must never grow, never panic, and never violate `len <= CAP`, no matter the
//!   sequence of pushes and pops thrown at it.
//!
//! Each test is a sentence about a guarantee. If one goes red, a safety property
//! the rest of the engine leans on has been lost.

use obelisk::game::entity::being::{Being, BeingError};
use obelisk::game::haps::{Event, Haps};

// ── The `.being` parser: hostile content never panics ──────────────────────

/// A number too large for its field comes back as a typed error, not a silent
/// wraparound — `i32::from_str` rejects the overflow before it can wrap.
#[test]
fn an_oversized_numeric_field_is_a_typed_error_not_an_overflow() {
	let huge = "name X\nhealth 99999999999999999999999999";
	match Being::parse(huge) {
		Err(BeingError::NotANumber { key, .. }) => assert_eq!(key, "health"),
		other => panic!("expected a typed NotANumber error, got {other:?}"),
	}
}

/// `bloom` is a `u16`; a negative or out-of-range value is rejected, never
/// truncated into a different number.
#[test]
fn an_out_of_range_unsigned_field_is_rejected_not_truncated() {
	assert!(matches!(
		Being::parse("name X\nbloom -1"),
		Err(BeingError::NotANumber { .. })
	));
	assert!(matches!(
		Being::parse("name X\nbloom 70000"),
		Err(BeingError::NotANumber { .. })
	));
}

/// A glyph is taken as one *character*, not one byte — a multibyte scalar must
/// not be sliced mid-codepoint (which would panic).
#[test]
fn a_multibyte_glyph_is_read_whole_not_sliced() {
	let being = Being::parse("name X\nglyph 世").expect("multibyte glyph parses");
	assert_eq!(being.glyph, '世');
}

/// Pathologically large input parses without panicking or stack-blowing.
#[test]
fn a_very_large_file_does_not_panic() {
	let mut text = String::from("name Huge\n---\n");
	text.push_str(&"A".repeat(200_000));
	let being = Being::parse(&text).expect("a large file still parses");
	assert_eq!(being.art.len(), 200_000);
}

/// Thousands of block separators are arrival data, not an error or a crash.
#[test]
fn a_storm_of_separators_does_not_panic() {
	let text = format!("name X\n{}", "---\n".repeat(5_000));
	assert!(Being::parse(&text).is_ok());
}

/// Windows line endings (`\r\n`) parse cleanly — the trailing `\r` is trimmed,
/// not carried into a value or a key.
#[test]
fn crlf_line_endings_parse_without_carrying_the_carriage_return() {
	let being = Being::parse("name Wisp\r\nglyph w\r\n").expect("crlf parses");
	assert_eq!(being.name, "Wisp");
	assert_eq!(being.glyph, 'w');
}

/// A header line that is only whitespace is skipped, not treated as a malformed
/// `key value` pair.
#[test]
fn a_whitespace_only_header_line_is_skipped_not_an_error() {
	let being = Being::parse("name X\n   \nglyph y").expect("blank-ish line is fine");
	assert_eq!(being.glyph, 'y');
}

/// Control bytes (including NUL) inside an art block are preserved verbatim, not
/// a parse failure or a panic — the art is kept exactly.
#[test]
fn control_bytes_in_an_art_block_are_preserved_not_panicked() {
	let being = Being::parse("name X\n---\na\0b\x07c").expect("control bytes parse");
	assert_eq!(being.art, "a\0b\x07c");
}

/// A header key with no value is a typed error, never an index panic.
#[test]
fn a_key_with_no_value_is_a_typed_error() {
	assert!(matches!(
		Being::parse("nameonly"),
		Err(BeingError::NotKeyValue(_))
	));
}

/// Empty input is the default being, not a crash — the floor case is safe.
#[test]
fn empty_input_yields_a_default_being() {
	let being = Being::parse("").expect("empty input is a default being");
	assert_eq!(being, Being { glyph: '?', visible: true, ..Being::default() });
}

// ── The event ring: bounded, panic-free, never grows ───────────────────────

/// Events are pure, copyable data (engine ward 1). This is a compile-time proof:
/// if `Event` ever grows a reference, lifetime, or heap field, it stops being
/// `Copy` and this file fails to build.
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
/// nothing is lost, duplicated, or reordered. Deterministic (a seeded LCG) so a
/// failure reproduces exactly.
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
			let event = Event::SpawnSekaikan { x: 0, y: 0, id: next_id };
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
			let expected = model.pop_front().map(|id| Event::SpawnSekaikan { x: 0, y: 0, id });
			assert_eq!(ring.pop(), expected, "the ring drains in first-in-first-out order");
		}

		assert!(ring.len() <= CAP, "len must never exceed capacity");
		assert_eq!(ring.len(), model.len(), "the ring and the model never diverge");
	}
}
