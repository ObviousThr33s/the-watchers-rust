//! The heightmap — the ground's altitude, and the whole of what the voxel
//! renderer needs to know about the land.
//!
//! The renderer asks the world exactly one question: *how high does the ground
//! stand at this cell?* That single `u8` ([`Heightmap`]) is the entire contract.
//! Anything else a surface might carry — water against rock, the line a place
//! surfaces when seen — is derived from the height or layered on above it, never
//! required here. Keeping the contract this thin is what lets a hand-built
//! fixture, a noise field, or a streamed chunk all feed one renderer that never
//! learns which it is looking at.

use noise::{NoiseFn, Perlin};

use crate::game::entity::PLAYER;
use crate::game::spaces::field::Field;

/// How high the ground stands at a world cell — the one fact the renderer reads.
/// Answered for *every* `(x, y)`, on-map or off, so a ray can march clean past
/// the edge of anything finite without a special case; the generator decides what
/// lies out there (open sea, endless hills, a wall of mountains).
pub trait Heightmap {
	fn height(&self, x: i16, y: i16) -> u8;
}

/// Any `Fn(i16, i16) -> u8` *is* a heightmap. This is the seam fixtures lean on —
/// a closure `|x, y| …` becomes a one-line patch of land — and it compiles away
/// to a direct call, costing nothing at runtime.
impl<F: Fn(i16, i16) -> u8> Heightmap for F {
	fn height(&self, x: i16, y: i16) -> u8 {
		self(x, y)
	}
}

/// The simple generator: Perlin noise lifted into a height. Deterministic for a
/// seed (the same seed always grows the same land), continuous so neighbours
/// differ by little, and bounded into `0..=amplitude`. Deliberately plain —
/// octaves, biomes, and the natural bounds (deep water, high plateaus) are layers
/// that come later, on top of this shape, not rewrites of it.
pub struct NoiseGround {
	perlin: Perlin,
	/// Spatial frequency: smaller spreads the hills wider, larger crinkles them.
	scale: f64,
	/// The tallest a hill may stand — noise's `[-1, 1]` maps onto `0..=amplitude`.
	amplitude: u8,
}

impl NoiseGround {
	/// A noise field seeded by `seed`, with gentle wide hills by default.
	pub fn new(seed: u32) -> Self {
		Self { perlin: Perlin::new(seed), scale: 0.05, amplitude: 12 }
	}

	/// Same field, but choose how broad the hills run and how tall they may stand.
	pub fn with(seed: u32, scale: f64, amplitude: u8) -> Self {
		Self { perlin: Perlin::new(seed), scale, amplitude }
	}
}

impl Heightmap for NoiseGround {
	fn height(&self, x: i16, y: i16) -> u8 {
		// Perlin returns roughly [-1, 1]; fold to [0, 1], then onto [0, amplitude].
		let n = self.perlin.get([x as f64 * self.scale, y as f64 * self.scale]);
		let lifted = (n + 1.0) / 2.0;
		(lifted * self.amplitude as f64).round().clamp(0.0, self.amplitude as f64) as u8
	}
}

/// How tall a solid field entity stands above the ground it sits on — enough to
/// clear the eye and read as a wall rather than a bump.
const WALL_RISE: u8 = 16;

/// The world's surface as the renderer sees it: the noise [`ground`](Self::ground),
/// with every solid thing standing in the [`field`](Self::field) — a wall, a tree —
/// raised into a tall column on top of it. This is the seam that ties the
/// first-person view to the Map: a wall *on the Map* becomes a wall *ahead of you*,
/// at the very same cell. The player is left flat — they are the camera, not a
/// structure, so their cell is never raised.
pub struct Surface<'a> {
	pub ground: &'a NoiseGround,
	pub field: &'a Field,
}

impl Heightmap for Surface<'_> {
	fn height(&self, x: i16, y: i16) -> u8 {
		let ground = self.ground.height(x, y);
		// A solid entity here (anything but the player) stands as a column on the
		// ground; an empty cell is just the ground.
		match self.field.get_entity_by_position(x, y) {
			Some(e) if e.id != PLAYER => ground.saturating_add(WALL_RISE),
			_ => ground,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::{Entity, Priority};

	#[test]
	fn a_closure_is_a_heightmap() {
		let pillar = |x: i16, y: i16| if (x, y) == (3, 0) { 10 } else { 0 };
		assert_eq!(pillar.height(3, 0), 10);
		assert_eq!(pillar.height(0, 0), 0, "off the pillar the ground is flat");
	}

	#[test]
	fn noise_is_deterministic_for_a_seed() {
		let a = NoiseGround::new(7);
		let b = NoiseGround::new(7);
		for (x, y) in [(0, 0), (3, 5), (-4, 9), (100, -100)] {
			assert_eq!(a.height(x, y), b.height(x, y), "same seed, same land at ({x},{y})");
		}
	}

	#[test]
	fn noise_stays_within_its_amplitude() {
		let g = NoiseGround::with(1, 0.1, 16);
		for x in -50..50 {
			for y in -50..50 {
				assert!(g.height(x, y) <= 16, "a hill never overtops its amplitude");
			}
		}
	}

	#[test]
	fn noise_actually_shapes_relief() {
		// A perfectly flat field would be a dead generator — it must rise and fall.
		let g = NoiseGround::new(3);
		let datum = g.height(0, 0);
		let differs = (-30..30).any(|x| g.height(x, 0) != datum);
		assert!(differs, "the land must have relief, not lie flat");
	}

	#[test]
	fn a_wall_stands_as_a_column_on_the_surface() {
		let ground = NoiseGround::new(1);
		let mut field = Field::new();
		field.add_entity(Entity::new(5, 5, '#', 1, Priority::LOW));
		let surface = Surface { ground: &ground, field: &field };

		assert!(
			surface.height(5, 5) > ground.height(5, 5),
			"a wall on the Map rises into a column ahead of you",
		);
		assert_eq!(
			surface.height(6, 6),
			ground.height(6, 6),
			"an empty cell is just the bare ground",
		);
	}

	#[test]
	fn the_player_is_not_a_wall() {
		let ground = NoiseGround::new(1);
		let mut field = Field::new();
		field.add_entity(Entity::new(2, 2, '^', PLAYER, Priority::MED));
		let surface = Surface { ground: &ground, field: &field };

		assert_eq!(
			surface.height(2, 2),
			ground.height(2, 2),
			"the camera's own cell stays flat — the player is no wall",
		);
	}
}
