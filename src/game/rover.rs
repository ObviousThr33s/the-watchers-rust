//! A rudimentary rover: a hull with a heading that turns in place and drives one
//! grid cell along where it points. Pure mechanism — it knows its position and
//! heading and nothing about the world; the caller says which cells are passable.
//!
//! Framed as a rover rather than a tank on purpose: it scouts and looks, which
//! suits a world built on watching better than something built to fight.

use std::f32::consts::{PI, TAU};

/// One eighth of a full turn — the rover's turn step, so its heading settles on
/// one of the eight grid directions and [`ahead`](Rover::ahead) rounds to a clean
/// cell.
pub const TURN_STEP: f32 = PI / 4.0;

/// A rover at a grid position, pointed along `heading` (ray-caster convention:
/// `0` is `+x`, and `+y` is down on screen).
#[derive(Clone)]
pub struct Rover {
	pub x: i16,
	pub y: i16,
	pub heading: f32,
}

impl Rover {
	/// Place a rover at `(x, y)` pointed along `heading`.
	pub fn new(x: i16, y: i16, heading: f32) -> Self {
		Rover { x, y, heading: wrap(heading) }
	}

	/// Turn the hull by `radians` (positive swings from `+x` toward `+y` — clockwise
	/// on screen). Position does not change; only where it points.
	pub fn turn(&mut self, radians: f32) {
		self.heading = wrap(self.heading + radians);
	}

	/// The cell one step ahead along the current heading, rounded to the grid.
	pub fn ahead(&self) -> (i16, i16) {
		let dx = self.heading.cos().round() as i16;
		let dy = self.heading.sin().round() as i16;
		(self.x + dx, self.y + dy)
	}

	/// Drive one cell forward along the heading, but only onto a cell `passable`
	/// allows. Returns whether it moved; a blocked rover keeps its place and its
	/// heading. The caller decides what is passable — walls, edges, other rovers —
	/// so this stays pure mechanism.
	pub fn drive(&mut self, passable: impl Fn(i16, i16) -> bool) -> bool {
		let (nx, ny) = self.ahead();
		if passable(nx, ny) {
			self.x = nx;
			self.y = ny;
			true
		} else {
			false
		}
	}
}

/// Wrap an angle into `(-PI, PI]`, matching the ray caster's convention.
fn wrap(mut a: f32) -> f32 {
	while a <= -PI {
		a += TAU;
	}
	while a > PI {
		a -= TAU;
	}
	a
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Everywhere is open — for tests about motion, not blocking.
	fn open(_x: i16, _y: i16) -> bool {
		true
	}

	#[test]
	fn turning_changes_heading_not_position() {
		let mut r = Rover::new(3, 3, 0.0);
		r.turn(TURN_STEP);
		assert_eq!((r.x, r.y), (3, 3), "a turn moves nothing, only the hull's aim");
		assert!((r.heading - TURN_STEP).abs() < 1e-5, "the heading swung by one step");
	}

	#[test]
	fn driving_advances_one_cell_along_the_heading() {
		// Facing +x (0 rad): one step ahead of the origin is (1, 0).
		let mut r = Rover::new(0, 0, 0.0);
		assert!(r.drive(open));
		assert_eq!((r.x, r.y), (1, 0), "a forward step follows where it points");
	}

	#[test]
	fn a_blocked_cell_stops_the_drive() {
		let mut r = Rover::new(0, 0, 0.0);
		let moved = r.drive(|_, _| false);
		assert!(!moved, "a wall ahead blocks the drive");
		assert_eq!((r.x, r.y), (0, 0), "a blocked rover keeps its cell");
	}

	#[test]
	fn turn_then_drive_goes_the_new_way() {
		// Start facing +x; two eighth-turns make a quarter, swinging aim to +y
		// (down on screen). Driving then steps in y, not x.
		let mut r = Rover::new(0, 0, 0.0);
		r.turn(TURN_STEP);
		r.turn(TURN_STEP);
		assert!(r.drive(open));
		assert_eq!((r.x, r.y), (0, 1), "after a quarter turn, forward is +y");
	}
}
