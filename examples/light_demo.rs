//! A prototype you can see: the polygon renderer's first slice, driven end to
//! end. Nothing here is mocked — it builds a real [`LightField`], deposits a
//! point light and a filled polygon into it, and quantises the float field down
//! to glyphs through the shipped `RAMP`. This is the spine *scene → light-field
//! → pixel* with the lens left out, exactly as far as the code is built today.
//!
//! Run it: `cargo run --example light_demo`

use obelisk::gfx::light::{LightField, RAMP};

fn main() {
	let mut field = LightField::new(54, 18);

	// A presence on the left: a point light, brightest at its source, falling
	// off with the square of distance. This is the being's radiation primitive.
	field.point(13.0, 9.0, 4.0);

	// A solid mass on the right: a filled polygon (a leaning diamond), its
	// interior flooded with even light. Light adds, so where the two meet they
	// would sum — here they are kept apart to read each primitive cleanly.
	field.fill_polygon(&[(40.0, 3.0), (49.0, 9.0), (40.0, 15.0), (33.0, 9.0)], 3.0);

	// The only place a "pixel" is chosen. Swap RAMP for half-blocks or braille
	// later and the optics above never notice.
	let screen = field.quantize(&RAMP);

	println!("Obelisk — light-field prototype  ({}x{})", field.width(), field.height());
	println!("ramp: {}", RAMP.iter().collect::<String>());
	println!("{}", "─".repeat(field.width() as usize));
	println!("{}", screen.to_string());
	println!("{}", "─".repeat(field.width() as usize));
	println!("a point light (left) and a filled polygon (right), real optics, no mocks");
}
