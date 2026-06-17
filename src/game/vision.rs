//! Line-of-sight / field-of-view visibility test — the foundation for the
//! "watchers move only when unwatched" mechanic. It answers one question:
//! can the viewer, facing a given direction, currently see a target cell?
//!
//! This is intentionally a pure function over a `blocked` predicate so it is
//! easy to test in isolation and reusable by both game logic and rendering.

/// Returns true if `target` is visible from `origin` while facing `facing`
/// (radians, same convention as the ray caster: 0 = +x, +y = down on screen).
/// All three gates must pass:
///
/// 1. **Range** — the target is no farther than `range` cells away.
/// 2. **Field of view** — the bearing to the target is within `fov / 2` of the
///    facing direction (things behind you don't count).
/// 3. **Line of sight** — no `blocked` cell lies between viewer and target
///    (walls hide whatever stands behind them).
///
/// `blocked(x, y)` reports whether a cell is opaque. The caller decides what
/// counts as opaque, and should exclude the viewer and the target themselves.
/// 
/// idk what you are doing, but think fluid and also. neat direction here. keep going.
pub fn can_see(
	origin: (i16, i16),
	facing: f32,
	fov: f32,
	range: f32,
	target: (i16, i16),
	blocked: impl Fn(i16, i16) -> bool,
) -> bool {
	use std::f32::consts::{PI, TAU};

	let (ox, oy) = (origin.0 as f32, origin.1 as f32);
	let (tx, ty) = (target.0 as f32, target.1 as f32);
	let (dx, dy) = (tx - ox, ty - oy);
	let dist = (dx * dx + dy * dy).sqrt();

	// Gate 1: range. (A distance of 0 means we're standing on it.)
	if dist > range {
		return false;
	}
	if dist == 0.0 {
		return true;
	}

	// Gate 2: field-of-view cone. Wrap the bearing difference into (-PI, PI].
	let mut diff = dy.atan2(dx) - facing;
	while diff <= -PI {
		diff += TAU;
	}
	while diff > PI {
		diff -= TAU;
	}
	if diff.abs() > fov / 2.0 {
		return false;
	}

	// Gate 3: line of sight. Walk the segment cell-by-cell (oversampled so we
	// can't step over a wall) and stop the moment something opaque blocks it.
	let steps = (dist * 2.0).ceil() as i32;
	for i in 1..steps {
		let t = i as f32 / steps as f32;
		let cell = ((ox + dx * t).round() as i16, (oy + dy * t).round() as i16);
		if cell == target {
			break;
		}
		if blocked(cell.0, cell.1) {
			return false;
		}
	}

	true
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;
	use std::f32::consts::PI;

	const FOV: f32 = PI / 3.0; // 60 degrees, matching the ray caster
	const RANGE: f32 = 20.0;

	fn walls(cells: &[(i16, i16)]) -> impl Fn(i16, i16) -> bool {
		let set: HashSet<(i16, i16)> = cells.iter().copied().collect();
		move |x, y| set.contains(&(x, y))
	}

	#[test]
	fn sees_target_in_clear_line_ahead() {
		// Facing +x with nothing in the way.
		assert!(can_see((0, 0), 0.0, FOV, RANGE, (5, 0), walls(&[])));
	}

	#[test]
	fn wall_between_blocks_sight() {
		// Same shot, but a wall sits between viewer and target.
		assert!(!can_see((0, 0), 0.0, FOV, RANGE, (5, 0), walls(&[(3, 0)])));
	}

	#[test]
	fn target_behind_viewer_is_unseen() {
		// Facing +x; the target is directly behind at -x.
		assert!(!can_see((0, 0), 0.0, FOV, RANGE, (-5, 0), walls(&[])));
	}

	#[test]
	fn target_beyond_range_is_unseen() {
		assert!(!can_see((0, 0), 0.0, FOV, 4.0, (5, 0), walls(&[])));
	}
}
