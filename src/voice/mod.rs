//! The void-voice: the game's player-facing transmission channel — a logger
//! turned outward.
//!
//! Where `utils::logger::Logger` records for the developer (timestamps, ticks,
//! a file on exit), `Voice` speaks to the *player*. Lines are queued, then
//! surfaced one at a time — the terminal trying to talk — and they corrupt as
//! the light fails. It is the channel for entity transmissions (Ooloonoo's
//! fragments, whatever the dark murmurs) and for the void at the open and close.

use std::collections::VecDeque;

/// Glyphs a transmission decays into as it is lost to noise.
const NOISE: &[char] = &['#', '%', '&', '@', '░', '▒', '▓', '?', '*', '/'];

#[derive(Default)]
pub struct Voice {
	/// Lines waiting to be spoken, oldest first.
	queue: VecDeque<String>,
	/// How garbled the channel is: 0.0 = clear, 1.0 = lost to noise.
	corruption: f32,
}

impl Voice {
	pub fn new() -> Self {
		Self::default()
	}

	/// Queue a line for the void to speak.
	pub fn transmit(&mut self, line: impl Into<String>) {
		self.queue.push_back(line.into());
	}

	/// Set how corrupted the channel is, clamped to `0.0..=1.0`. Drive this from
	/// the failing light: less light, more noise.
	pub fn set_corruption(&mut self, amount: f32) {
		self.corruption = amount.clamp(0.0, 1.0);
	}

	/// Take the next line, garbled to the current corruption level. `None` when
	/// the void has nothing more to say.
	pub fn next_line(&mut self) -> Option<String> {
		let line = self.queue.pop_front()?;
		Some(corrupt(&line, self.corruption))
	}

	/// How many lines are still waiting to be spoken.
	pub fn pending(&self) -> usize {
		self.queue.len()
	}
}

/// Deterministic pseudo-random value in `[0.0, 1.0)` for position `i`, so a
/// corrupted transmission renders the same way every frame.
fn noise_at(i: usize) -> f32 {
	let x = (i as u64).wrapping_mul(2654435761).wrapping_add(0x9E3779B9);
	((x >> 16) & 0xFFFF) as f32 / 65536.0
}

/// Replaces a fraction of `line`'s glyphs with noise, scaled by `level`. Spaces
/// are kept so the shape of the words survives even as the content dissolves.
fn corrupt(line: &str, level: f32) -> String {
	if level <= 0.0 {
		return line.to_string();
	}

	line.chars()
		.enumerate()
		.map(|(i, c)| {
			if c == ' ' {
				c
			} else if level >= 1.0 || noise_at(i) < level {
				let idx = (noise_at(i.wrapping_add(7)) * NOISE.len() as f32) as usize;
				NOISE[idx.min(NOISE.len() - 1)]
			} else {
				c
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn speaks_in_order() {
		let mut v = Voice::new();
		v.transmit("first");
		v.transmit("second");
		assert_eq!(v.pending(), 2);
		assert_eq!(v.next_line().as_deref(), Some("first"));
		assert_eq!(v.next_line().as_deref(), Some("second"));
		assert_eq!(v.next_line(), None);
	}

	#[test]
	fn clear_channel_is_untouched() {
		let mut v = Voice::new(); // corruption defaults to 0.0
		v.transmit("you are seen");
		assert_eq!(v.next_line().as_deref(), Some("you are seen"));
	}

	#[test]
	fn full_corruption_keeps_shape_but_loses_the_words() {
		let mut v = Voice::new();
		v.set_corruption(1.0);
		v.transmit("hello void");
		let spoken = v.next_line().unwrap();

		// One glyph in, one glyph out; spaces survive; no original letters left.
		assert_eq!(spoken.chars().count(), "hello void".chars().count());
		assert!(spoken.contains(' '));
		assert!(!spoken.contains('h'));
		assert!(spoken.chars().filter(|c| *c != ' ').all(|c| NOISE.contains(&c)));
	}

	#[test]
	fn corruption_clamps() {
		let mut v = Voice::new();
		v.set_corruption(5.0); // clamped to 1.0, no panic
		v.transmit("x");
		assert!(NOISE.contains(&v.next_line().unwrap().chars().next().unwrap()));
	}
}
