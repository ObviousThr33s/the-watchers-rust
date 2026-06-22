//! Shared fixtures for the integration-test crates.
//!
//! Per *The Rust Programming Language* (ch. 11, "Submodules in Integration
//! Tests"), helper code shared across test files lives in `tests/common/mod.rs`.
//! The `…/mod.rs` form is deliberate: a flat `tests/common.rs` would be compiled
//! as its own test crate and surface as a stray empty test run. As
//! `common/mod.rs`, Cargo treats it as an ordinary module that other test crates
//! pull in with `mod common;`, and it is never run as a test on its own.

use obelisk::game::entity::EntityId;
use obelisk::game::haps::Event;

/// A spawn event addressed by `id`, with position pinned to the origin — the
/// reusable fixture for event-ring tests. Identity is the only thing those tests
/// turn on, so `x`/`y` are fixed and out of the way.
pub fn spawn(id: EntityId) -> Event {
	Event::SpawnSekaikan { x: 0, y: 0, id }
}
