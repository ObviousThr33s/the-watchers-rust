//! Terrain — sowing the field with its flora. Pure mechanism: it loads the flora
//! *definitions* from their `.being` files (a tree's glyph, art, and line live
//! there, never here) and scatters them across the field as solid entities.
//! *What* a tree is, is data; *where* the trees fall is the only thing this
//! module decides.
//!
//! The scatter is Perlin noise thresholded into groves and open glades, not a
//! uniform sprinkle — so a forest reads like terrain rather than confetti. A
//! given `seed` always grows the same forest, which keeps it testable.

use std::path::Path;

use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game::entity::being::Being;
use crate::game::entity::{Entity, Priority};
use crate::game::spaces::field::Field;

/// Where the flora definitions live. One folder, scanned at sow time, so dropping
/// a new `.being` in it is all it takes to add a species to the world — no code.
pub const FLORA_DIR: &str = "res/entities/flora";

/// How a patch of forest is sown. Every knob in one place, so the call site reads
/// like a description of the grove instead of a row of loose numbers.
#[derive(Clone, Copy, Debug)]
pub struct Sowing {
	/// The rectangle to sow, in field coordinates: `[x0, x0 + width) x [y0, y0 + height)`.
	pub x0: i16,
	pub y0: i16,
	pub width: i16,
	pub height: i16,
	/// Cells within this Chebyshev distance of `clear_around` are left bare, so
	/// the player isn't planted inside a thicket at spawn.
	pub clear_around: (i16, i16),
	pub clearing: i16,
	/// Perlin threshold in roughly `[-1, 1]`: flora grows where the noise rises
	/// above it. Lower means a denser forest, higher means more open ground.
	pub threshold: f64,
	/// Spatial frequency of the noise. Smaller values make broader groves and
	/// wider clearings; larger values break the canopy up into scattered clumps.
	pub scale: f64,
	/// Seeds both the grove shape (noise) and the species choice (rng), so the
	/// same seed always grows the same forest.
	pub seed: u32,
}

/// Loads every `.being` definition under [`FLORA_DIR`].
pub fn load_flora() -> Vec<Being> {
	load_flora_from(Path::new(FLORA_DIR))
}

/// Loads every `.being` definition in `dir`. A missing folder yields no flora
/// (the world simply grows nothing), and a single malformed file is skipped
/// rather than fatal — one bad tree must not stop the forest. The
/// `every_flora_file_parses` guard test is what keeps that skip from hiding a
/// real breakage.
pub fn load_flora_from(dir: &Path) -> Vec<Being> {
	let mut flora = Vec::new();
	let entries = match std::fs::read_dir(dir) {
		Ok(entries) => entries,
		Err(_) => return flora,
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) == Some("being") {
			if let Ok(being) = Being::load(&path) {
				flora.push(being);
			}
		}
	}
	flora
}

/// Sows `flora` across `plan`'s rectangle and returns how many were planted.
///
/// A cell grows a tree only when all three hold: it falls outside the clearing,
/// nothing already stands there (the sow never overwrites the player or a wall),
/// and the noise rises above the threshold. Flora are solid — the ray caster
/// renders every entity as a wall — so a planted tree is at once scenery on the
/// Map and something the first-person view can cast against and you can bump.
pub fn sow(field: &mut Field, flora: &[Being], plan: Sowing) -> usize {
	if flora.is_empty() {
		return 0;
	}

	let perlin = Perlin::new(plan.seed);
	let mut rng = StdRng::seed_from_u64(plan.seed.into());
	let (cx, cy) = plan.clear_around;
	let mut planted = 0;

	for y in plan.y0..plan.y0.saturating_add(plan.height) {
		for x in plan.x0..plan.x0.saturating_add(plan.width) {
			// Keep the spawn clearing open.
			if (x - cx).abs() <= plan.clearing && (y - cy).abs() <= plan.clearing {
				continue;
			}
			// Never plant over something already standing here.
			if field.get_entity_by_position(x, y).is_some() {
				continue;
			}
			// Groves, not confetti: flora only where the noise crests the threshold.
			let density = perlin.get([x as f64 * plan.scale, y as f64 * plan.scale]);
			if density <= plan.threshold {
				continue;
			}

			let Some(being) = flora.get(rng.random_range(0..flora.len())) else {
				continue;
			};
			field.add_entity(Entity::new(
				x,
				y,
				being.glyph,
				format!("{}_{x}_{y}", being.name),
				Priority::LOW,
			));
			planted += 1;
		}
	}

	planted
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashSet;

	/// A plain sowing plan over a clear field, with the clearing pushed far off so
	/// it doesn't interfere with whatever a test is measuring.
	fn plan() -> Sowing {
		Sowing {
			x0: 0,
			y0: 0,
			width: 40,
			height: 24,
			clear_around: (-999, -999),
			clearing: 0,
			threshold: 0.1,
			scale: 0.18,
			seed: 1,
		}
	}

	#[test]
	fn loads_the_real_flora_folder() {
		let flora = load_flora();
		assert!(flora.len() >= 3, "expected several flora definitions, found {}", flora.len());
		assert!(
			flora.iter().all(|b| !b.name.is_empty() && b.glyph != '\0'),
			"every loaded flora should carry a name and a glyph",
		);
	}

	#[test]
	fn missing_folder_grows_nothing() {
		assert!(load_flora_from(Path::new("res/entities/no-such-folder")).is_empty());
	}

	#[test]
	fn sow_plants_flora_into_the_field() {
		let flora = load_flora();
		let mut field = Field::new();
		let planted = sow(&mut field, &flora, plan());

		assert!(planted > 0, "a forest should actually grow");
		assert_eq!(planted, field.entities.len(), "every planted tree lands in the field");

		// Only the loaded species are ever placed — the engine invents nothing.
		let glyphs: HashSet<char> = flora.iter().map(|b| b.glyph).collect();
		assert!(
			field.entities.values().all(|e| glyphs.contains(&e.self_)),
			"every planted glyph must come from a flora definition",
		);
	}

	#[test]
	fn sow_is_deterministic_for_a_seed() {
		let flora = load_flora();
		let (mut a, mut b) = (Field::new(), Field::new());
		sow(&mut a, &flora, plan());
		sow(&mut b, &flora, plan());

		let cells = |f: &Field| -> Vec<(i16, i16, char)> {
			let mut v: Vec<_> = f.entities.values().map(|e| (e.x, e.y, e.self_)).collect();
			v.sort();
			v
		};
		assert_eq!(cells(&a), cells(&b), "the same seed must grow the same forest");
	}

	#[test]
	fn sow_keeps_the_clearing_bare() {
		let flora = load_flora();
		let mut field = Field::new();
		let plan = Sowing { clear_around: (5, 5), clearing: 3, threshold: -1.0, ..plan() };
		sow(&mut field, &flora, plan);

		for y in 2..=8 {
			for x in 2..=8 {
				assert!(
					field.get_entity_by_position(x, y).is_none(),
					"({x},{y}) lies in the clearing and must stay bare",
				);
			}
		}
	}

	#[test]
	fn sow_never_overwrites_what_already_stands() {
		let flora = load_flora();
		let mut field = Field::new();
		// A wall in the middle of the patch: the sow must plant around it, not over.
		field.add_entity(Entity::new(10, 10, '#', "wall".to_string(), Priority::LOW));

		let plan = Sowing { threshold: -1.0, ..plan() }; // try to fill every cell
		sow(&mut field, &flora, plan);

		assert_eq!(
			field.get_entity_by_position(10, 10).map(|e| e.self_),
			Some('#'),
			"an existing entity must survive the sow",
		);
	}
}
