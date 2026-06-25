//! Absolute-safety tests for the polygon path — the *do not call upon or in*
//! contract, in the maker's own words: **never invoke anything endless or
//! nameless.** A `.poly` file is content, authored by anyone. The loader reads
//! and parses but never *resolves* a reference, and the renderer crosses a poly
//! into light by its slot *count* alone. So no file and no shape can make this
//! path recurse without a floor (the endless) or dereference a slot into the
//! thing it names (the nameless).
//!
//! Integration tests (ch. 11.3): public API only — the linked front door, never
//! alone in the internals. Unit tests for the mechanics live inline in
//! `src/game/poly.rs` and `src/gfx/light.rs`; these are the value-wards on top.
//! Each test below is a sentence about a guarantee; a red one means a boundary
//! the maker drew has been crossed.

use obelisk::game::poly::{Poly, Slot};
use obelisk::gfx::light::{regular_polygon, LightField};

/// The nameless is met but never invoked. A lone `{Triad}` is an anonymous
/// reference — a slot, not a shape. `parse_all` passes it over: it is never
/// returned as a polygon and never resolved into the `Triad` it names.
#[test]
fn the_nameless_reference_is_met_but_never_invoked() {
	let polys = Poly::parse_all("#cyclic\n{Triad}\nPentad{{}{}{}{}{}}").expect("parses");
	assert_eq!(polys.len(), 1, "only the named polygon loads; the nameless reference is dropped");
	assert_eq!(polys.first().map(|p| p.name.as_str()), Some("Pentad"));
}

/// The endless is never entered. The self-referential `$Polyad{{poly}{}}` loads
/// with its `poly` slot kept as a *name* — parse never follows it into itself,
/// so there is no recursion without a floor.
#[test]
fn the_self_referential_polygon_keeps_its_slot_a_name() {
	let polys = Poly::parse_all("$Polyad{{poly}{}}").expect("parses");
	let polyad = polys.iter().find(|p| p.name == "$Polyad").expect("the polyad loads");
	assert_eq!(
		polyad.slots,
		vec![Slot::Filled("poly".into()), Slot::Empty],
		"the cyclic stays a name in the mirror, never dereferenced",
	);
}

/// Endless-shaped *input* is met without an endless *process*. A pathologically
/// deep nesting is parsed by an iterative walk, so it returns a typed result and
/// never blows the stack.
#[test]
fn a_pathologically_nested_poly_is_met_without_endless_recursion() {
	let depth = 50_000;
	let deep = format!("{}{}", "{".repeat(depth), "}".repeat(depth));
	assert!(
		matches!(Poly::parse_all(&deep), Ok(_) | Err(_)),
		"deep nesting returns — it is never entered as a recursion",
	);
}

/// Crossing to light invokes only the *count*. The two-slot `$Polyad` becomes two
/// points — no polygon — so both the fill and the wireframe deposit nothing. The
/// `poly` slot is never read on the way to the screen.
#[test]
fn rendering_the_cyclic_takes_only_the_count_and_lights_nothing() {
	let polys = Poly::parse_all("$Polyad{{poly}{}}").expect("parses");
	let polyad = polys.iter().find(|p| p.name == "$Polyad").expect("the polyad loads");
	let verts = regular_polygon(polyad.slots.len(), 10.0, 10.0, 6.0, 0.0);
	assert!(verts.is_empty(), "two slots make no polygon");

	let mut field = LightField::new(20, 20);
	field.fill_polygon(&verts, 1.0);
	field.outline_polygon(&verts, 1.0);
	assert_eq!(field.max(), 0.0, "the cyclic deposits no light, filled or wireframe");
}

/// A hostile coordinate cannot blow the field. The light-field ignores off-field
/// deposits, so vertices flung far outside simply land nowhere — the visible
/// field is the cap, the line where cheap and safe are the same.
#[test]
fn a_far_flung_polygon_cannot_grow_the_field() {
	let mut field = LightField::new(16, 16);
	field.fill_polygon(&[(1.0e9, 1.0e9), (2.0e9, 1.0e9), (1.5e9, 2.0e9)], 1.0);
	assert_eq!(field.max(), 0.0, "nothing off the field is ever stored");
}
