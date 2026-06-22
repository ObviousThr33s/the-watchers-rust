// Only hide the console for real builds; keep it for `cargo test` so the test
// runner has somewhere to print (otherwise results vanish on Windows).
#![cfg_attr(not(test), windows_subsystem = "windows")]

pub mod utils;
pub mod loops;
pub mod game;
pub mod gfx;
pub mod input;
pub mod voice;

/// The product's display name — the terminal you ship. Kept as one source of
/// truth so the name lives in a single place, never scattered through string
/// literals. The crate itself stays `the-watchers-rust`: that is the *engine*.
/// Splitting the two later (engine keeps its name, the product is Obelisk) is a
/// branch we can explore — this constant is the seam that makes it cheap.
pub const NAME: &str = "Obelisk";