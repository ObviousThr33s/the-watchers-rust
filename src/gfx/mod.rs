//! Graphics: turning game state into what the terminal shows. The pipeline is
//! [`screen`] (a layered cell buffer) over [`light`] (an `f32` light-field), with
//! [`viewport`] casting the first-person view, [`minimap`] painting the map, and
//! [`ui`] laying the panels out through ratatui. This module is the thin entry
//! point the main loop calls each frame.

pub mod ui;
pub mod minimap;
pub mod portal;
pub mod viewport;
pub mod voxel;

pub mod screen;
pub mod light;

use crate::{game::spaces::field::Field, utils::logger::Logger};
pub use viewport::Viewport;
use portal::Portal;

/// Draw one frame: hand the whole UI — first-person view, map, log, and panels —
/// to ratatui to lay out and paint. A failed draw is swallowed, since a dropped
/// frame beats a panic mid-render.
pub fn render(terminal: &mut ratatui::DefaultTerminal, log:&Logger, field:&Field, viewport_text:&String, player_pos:(i16, i16), portal:&Portal, inventory:&[crate::game::item::Item], scroll:u16) {
	//thank you ratatui for TUI-ing so I could TUI something cool with the robots.
	let _ = terminal.draw(|frame| ui::draw_(frame, viewport_text, field, log, player_pos, portal, inventory, scroll) );

}

/// Wipe the terminal — e.g. before a fresh draw. A failed clear is ignored for
/// the same reason as [`render`].
pub fn clear(terminal: &mut ratatui::DefaultTerminal){
	let _ = terminal.clear();
}

