//! Shared fixtures for the integration-test crates.
//!
//! Per *The Rust Programming Language* (ch. 11, "Submodules in Integration
//! Tests"), helper code shared across test files lives in `tests/common/mod.rs`.
//! The `…/mod.rs` form is deliberate: a flat `tests/common.rs` would be compiled
//! as its own test crate and surface as a stray empty test run. As
//! `common/mod.rs`, Cargo treats it as an ordinary module that other test crates
//! pull in with `mod common;`, and it is never run as a test on its own.

// PARKED — this module held `spawn(id)`, the value-bearing event fixture the
// identity-FIFO property test in `safety_haps.rs` turned on. The SpawnSekaikan
// retreat removed the only `Event` variant that carried a value, so the fixture
// has nothing to mint. No test crate currently pulls this module in (`mod common;`
// is gone from `safety_haps.rs`). Restore both together when a value-carrying
// Event returns; the original fixture is in git history.
