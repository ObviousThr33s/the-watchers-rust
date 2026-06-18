// Only hide the console for real builds; keep it for `cargo test` so the test
// runner has somewhere to print (otherwise results vanish on Windows).
#![cfg_attr(not(test), windows_subsystem = "windows")]

pub mod utils;
pub mod loops;
pub mod game;
pub mod gfx;
pub mod input;
pub mod voice;