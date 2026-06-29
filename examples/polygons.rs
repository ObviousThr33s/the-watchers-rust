//! See the polygons. Loads the real `res/entities/poly.poly`, crosses each shape
//! into light by its slot *count* alone, and prints the glyph field — so you can
//! look at a `.poly` and see a shape. Run: `cargo run --example polygons`.
//!
//! The self-referential `$Polyad` (two slots) is met as the number two: no
//! polygon, a dark field, never called upon. You watch the cage hold.

use obelisk::game::poly::Poly;
use obelisk::gfx::light::{regular_polygon, LightField, RAMP};

fn main() {
	let path = "res/entities/poly.poly";
	let polys = match Poly::load(path) {
		Ok(p) => p,
		Err(e) => {
			eprintln!("could not load {path}: {e}");
			return;
		}
	};

	let (w, h) = (24u16, 12u16);
	let (cx, cy) = (f32::from(w) / 2.0, f32::from(h) / 2.0);

	for poly in &polys {
		let name = if poly.name.is_empty() { "(anon)" } else { poly.name.as_str() };
		let sides = poly.slots.len();

		let mut field = LightField::new(w, h);
		// First vertex points up (-y); count only, the slots are never read.
		field.fill_polygon(&regular_polygon(sides, cx, cy, 5.0, -std::f32::consts::FRAC_PI_2), 1.0);

		println!("\n{name} — {sides} slots");
		if sides < 3 {
			println!("  (no polygon — met as the number {sides}, never called upon)");
		} else {
			println!("{}", field.quantize(&RAMP).to_string());
		}
	}

	// The same shapes as an Elite-style wireframe — only the vector outline is lit,
	// the interior left dark. This is the look meant for the main viewport.
	for poly in &polys {
		let sides = poly.slots.len();
		if sides < 3 {
			continue;
		}
		let name = if poly.name.is_empty() { "(anon)" } else { poly.name.as_str() };
		let mut field = LightField::new(w, h);
		field.outline_polygon(&regular_polygon(sides, cx, cy, 5.0, -std::f32::consts::FRAC_PI_2), 1.0);
		println!("\n{name} — wireframe");
		println!("{}", field.quantize(&RAMP).to_string());
	}

	// A scene: more than one polygon in one field. Light *accumulates*, so where two
	// shapes overlap the glyphs deepen — many polygons becoming a single picture.
	let renderable: Vec<usize> = polys.iter().map(|p| p.slots.len()).filter(|&n| n >= 3).collect();
	let (sw, sh) = (40u16, 16u16);
	let mut scene = LightField::new(sw, sh);
	let up = -std::f32::consts::FRAC_PI_2;
	let mut sx = 13.0_f32;
	for &sides in &renderable {
		scene.fill_polygon(&regular_polygon(sides, sx, f32::from(sh) / 2.0, 5.0, up), 1.0);
		sx += 8.0;
	}
	println!("\nscene — every polygon in the file, brought into one field");
	println!("{}", scene.quantize(&RAMP).to_string());
}
