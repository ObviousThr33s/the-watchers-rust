//! The light-field: the renderer's substrate, one layer beneath the cells you
//! see. Where [`Screen`](crate::gfx::screen::Screen) holds glyphs, this holds
//! *light* — a grid of `f32` radiance, continuous rather than quantised.
//!
//! Everything optical is meant to happen here, in float space, before anything
//! becomes a character: a source deposits brightness that falls off with the
//! square of distance (the inverse-square law); a polygon floods its interior.
//! Only at the very end is the field quantised down to glyphs — and that last
//! step is deliberately swappable, so the choice of "pixel" (a brightness ramp
//! now; half-blocks or braille later) never dictates the optics above it.
//!
//! This is the seam the whole renderer hangs from: *scene → light-field → (lens)
//! → pixel*. The lens — a warp of where we sample, and filters over the field
//! (vignette, blur, the corruption of memory) — lands on top of this later.

use crate::gfx::screen::Screen;

/// The default brightness ramp, dark to light. A dark cell maps to a space —
/// bare paper, left so the layer behind shows through, the same negative space
/// (*ma*) [`Screen`] is built around.
pub const RAMP: [char; 6] = [' ', '·', '░', '▒', '▓', '█'];

/// A grid of `f32` radiance. Light is *accumulated*, not overwritten, so
/// overlapping sources and polygons add up the way real light does.
pub struct LightField {
	width: u16,
	height: u16,
	/// Row-major radiance, one `f32` per cell.
	lux: Vec<f32>,
}

impl LightField {
	/// A dark field of the given size — every cell at zero radiance.
	pub fn new(width: u16, height: u16) -> Self {
		LightField {
			width,
			height,
			lux: vec![0.0; usize::from(width) * usize::from(height)],
		}
	}

	pub fn width(&self) -> u16 { self.width }
	pub fn height(&self) -> u16 { self.height }

	/// Row-major index of `(x, y)`, or `None` if it lies off the field.
	fn index(&self, x: u16, y: u16) -> Option<usize> {
		if x < self.width && y < self.height {
			Some(usize::from(y) * usize::from(self.width) + usize::from(x))
		} else {
			None
		}
	}

	/// Radiance at `(x, y)`. Off-field cells read as dark.
	pub fn get(&self, x: u16, y: u16) -> f32 {
		match self.index(x, y) {
			Some(i) => self.lux[i],
			None => 0.0,
		}
	}

	/// Deposit `v` more radiance at cell `(x, y)`. Off-field coordinates are
	/// ignored, so callers never bounds-check before adding light.
	pub fn add(&mut self, x: u16, y: u16, v: f32) {
		if let Some(i) = self.index(x, y) {
			self.lux[i] += v;
		}
	}

	/// Reset the whole field to dark.
	pub fn clear(&mut self) {
		for cell in &mut self.lux {
			*cell = 0.0;
		}
	}

	/// The brightest cell's radiance — the peak the quantiser normalises against.
	pub fn max(&self) -> f32 {
		self.lux.iter().copied().fold(0.0_f32, f32::max)
	}

	/// Deposit a point light at float position `(cx, cy)`, brightening every cell
	/// by the inverse-square law `power / (1 + r²)`. The `+1` softens the
	/// singularity at the source, so the centre is finite (≈ `power`) instead of
	/// infinite. Light accumulates, so two sources sum where they overlap. This
	/// is the primitive the being's radiation — a presence pricking light into
	/// the field — is built from.
	pub fn point(&mut self, cx: f32, cy: f32, power: f32) {
		for y in 0..self.height {
			for x in 0..self.width {
				let dx = f32::from(x) + 0.5 - cx;
				let dy = f32::from(y) + 0.5 - cy;
				let r2 = dx * dx + dy * dy;
				self.add(x, y, power / (1.0 + r2));
			}
		}
	}

	/// Flood a polygon's interior with `intensity`, by even-odd scanline fill.
	/// Vertices are float cell-space points, walked as a closed loop (the last
	/// joins back to the first). Convex and concave simple polygons both fill
	/// correctly. Light is added, not set, so a polygon brightens whatever it
	/// already covers — this is how a scene "made of polygons" deposits into the
	/// field.
	pub fn fill_polygon(&mut self, verts: &[(f32, f32)], intensity: f32) {
		if verts.len() < 3 {
			return;
		}
		let n = verts.len();
		for y in 0..self.height {
			let yc = f32::from(y) + 0.5;
			// x at which each straddling edge crosses this scanline.
			let mut crossings: Vec<f32> = Vec::new();
			for i in 0..n {
				let (x0, y0) = verts[i];
				let (x1, y1) = verts[(i + 1) % n];
				// Half-open test: an edge counts only when the scanline falls
				// between its endpoints, so a shared vertex isn't double-counted
				// and horizontal edges (which never straddle) are skipped.
				if (y0 <= yc) != (y1 <= yc) {
					let t = (yc - y0) / (y1 - y0);
					crossings.push(x0 + t * (x1 - x0));
				}
			}
			crossings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
			// Interior spans run between successive crossing pairs.
			for span in crossings.chunks_exact(2) {
				if let [xs, xe] = span {
					for x in 0..self.width {
						let xc = f32::from(x) + 0.5;
						if xc >= *xs && xc < *xe {
							self.add(x, y, intensity);
						}
					}
				}
			}
		}
	}

	/// Draw a straight segment of light from `a` to `b`, one step per cell along
	/// the longer axis (a DDA walk). This is the edge primitive a wireframe is made
	/// of — Elite's vectors, not filled mass. Off-field cells are ignored by
	/// [`add`](Self::add), so a segment may run to the border without a bounds check.
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // float walk, then deposit at integer cells
	pub fn line(&mut self, a: (f32, f32), b: (f32, f32), intensity: f32) {
		let (x0, y0) = a;
		let (x1, y1) = b;
		let (dx, dy) = (x1 - x0, y1 - y0);
		let steps = dx.abs().max(dy.abs()).ceil().max(1.0);
		let n = steps as usize;
		for i in 0..=n {
			let t = i as f32 / steps;
			let (x, y) = (x0 + dx * t, y0 + dy * t);
			if x >= 0.0 && y >= 0.0 {
				self.add(x as u16, y as u16, intensity);
			}
		}
	}

	/// Trace a polygon's edges as a closed loop — the wireframe: only the vector
	/// outline is lit, the interior left dark (Elite, not the filled mass above).
	/// Like the fill, it takes already-resolved points; the slots that named them
	/// are never read here, and `regular_polygon` already caged the cyclic upstream.
	pub fn outline_polygon(&mut self, verts: &[(f32, f32)], intensity: f32) {
		let n = verts.len();
		if n < 2 {
			return;
		}
		for i in 0..n {
			self.line(verts[i], verts[(i + 1) % n], intensity);
		}
	}

	/// Quantise the float field down to glyphs — the renderer's last step, and
	/// the only place a "pixel" is chosen. Radiance is normalised against the
	/// brightest cell, then mapped onto `ramp` (dark → light). Swapping this ramp
	/// — or this whole method — for half-blocks or braille is how the pixel
	/// decision stays deferred: nothing in the optics above depends on it.
	pub fn quantize(&self, ramp: &[char]) -> Screen {
		let mut screen = Screen::new(self.width, self.height);
		let peak = self.max();
		if ramp.is_empty() || peak <= 0.0 {
			return screen;
		}
		for y in 0..self.height {
			for x in 0..self.width {
				let level = (self.get(x, y) / peak).clamp(0.0, 1.0);
				let band = (level * ramp.len() as f32) as usize;
				let glyph = ramp[band.min(ramp.len() - 1)];
				// A dark glyph (a space) is left as bare paper, never painted, so
				// the layer behind keeps showing through.
				if glyph != ' ' {
					screen.put(x, y, glyph);
				}
			}
		}
		screen
	}
}

/// The vertices of a regular polygon — `sides` points spaced evenly on a circle
/// of `radius` about `(cx, cy)`, the first at angle `phase` (radians). This is the
/// bridge a symbolic [`Poly`](crate::game::poly::Poly) crosses to become light: a
/// caller hands over the *count* of a poly's slots and gets back a shape to flood
/// with [`fill_polygon`](LightField::fill_polygon).
///
/// The signature is the cage. It takes only how *many* vertices there are, never
/// the slots themselves — so a self-referential polygon is drawn by its arity and
/// never by what its slots name. There is nothing here to call upon or in; the
/// cyclic is met as a number. Fewer than three points is no polygon and yields
/// none, so a degenerate shape (the two-slot `$Polyad`) simply lights nothing.
#[allow(clippy::cast_precision_loss)] // small vertex counts; the angle stays exact enough
pub fn regular_polygon(sides: usize, cx: f32, cy: f32, radius: f32, phase: f32) -> Vec<(f32, f32)> {
	if sides < 3 {
		return Vec::new();
	}
	(0..sides)
		.map(|i| {
			let theta = phase + std::f32::consts::TAU * i as f32 / sides as f32;
			(cx + radius * theta.cos(), cy + radius * theta.sin())
		})
		.collect()
}

// Tests are the spec: each one states a property the light-field must hold.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_point_light_is_brightest_at_its_source() {
		let mut field = LightField::new(9, 9);
		field.point(4.5, 4.5, 1.0); // the centre of cell (4, 4)
		let centre = field.get(4, 4);
		let edge = field.get(0, 0);
		assert!(centre > edge, "a light must be brightest where it sits");
		assert!(centre > 0.9, "the softened centre is ≈ the source power");
	}

	#[test]
	fn falloff_weakens_with_distance() {
		let mut field = LightField::new(21, 1);
		field.point(0.5, 0.5, 1.0); // a source at cell (0, 0)
		let near = field.get(1, 0);
		let mid = field.get(5, 0);
		let far = field.get(10, 0);
		assert!(near > mid && mid > far, "light falls off the further you go");
	}

	#[test]
	fn light_accumulates_where_sources_overlap() {
		let mut field = LightField::new(5, 5);
		field.point(2.5, 2.5, 1.0);
		let single = field.get(2, 2);
		field.point(2.5, 2.5, 1.0); // a second source on the same spot
		let doubled = field.get(2, 2);
		assert!((doubled - 2.0 * single).abs() < 1e-4, "two lights sum");
	}

	#[test]
	fn a_filled_polygon_floods_its_interior_and_spares_the_outside() {
		let mut field = LightField::new(10, 10);
		// A square covering the cells from (2,2) to (8,8).
		field.fill_polygon(&[(2.0, 2.0), (8.0, 2.0), (8.0, 8.0), (2.0, 8.0)], 1.0);
		assert!(field.get(5, 5) > 0.0, "the interior is lit");
		assert_eq!(field.get(0, 0), 0.0, "a cell outside the polygon stays dark");
	}

	#[test]
	fn quantise_maps_bright_to_the_ramp_top_and_dark_to_bare_paper() {
		let mut field = LightField::new(3, 1);
		field.add(2, 0, 1.0); // one bright cell, the rest dark
		let row = field.quantize(&RAMP).to_string();
		assert_eq!(row.chars().next(), Some(' '), "a dark cell is left as bare paper");
		assert_eq!(row.chars().nth(2), Some('█'), "the brightest cell hits the ramp top");
	}

	#[test]
	fn a_regular_polygon_places_one_vertex_per_side_on_the_circle() {
		let verts = regular_polygon(5, 10.0, 10.0, 6.0, 0.0);
		assert_eq!(verts.len(), 5, "five sides, five contact points");
		for (x, y) in verts {
			let r = ((x - 10.0).powi(2) + (y - 10.0).powi(2)).sqrt();
			assert!((r - 6.0).abs() < 1e-3, "every vertex sits on the circle of radius 6");
		}
	}

	#[test]
	fn fewer_than_three_slots_make_no_polygon() {
		assert!(regular_polygon(2, 5.0, 5.0, 3.0, 0.0).is_empty(), "two points are a line, not a shape");
		assert!(regular_polygon(0, 5.0, 5.0, 3.0, 0.0).is_empty(), "nothing is no shape");
	}

	/// The crossing, end to end: a polygon read from the real `.poly` file becomes
	/// light. The `Pentad` (five slots) is drawn as a pentagon and floods its
	/// interior — proof a symbolic node reaches the field through its *count* alone.
	#[test]
	fn a_poly_from_the_file_crosses_into_light() {
		use crate::game::poly::Poly;
		let polys = Poly::load("res/entities/poly.poly").expect("load poly.poly");
		let pentad = polys.iter().find(|p| p.name == "Pentad").expect("the file holds a Pentad");

		let mut field = LightField::new(20, 20);
		field.fill_polygon(&regular_polygon(pentad.slots.len(), 10.0, 10.0, 6.0, 0.0), 1.0);
		assert!(field.get(10, 10) > 0.0, "the pentagon floods its centre with light");
	}

	/// The cyclic stays caged in the renderer too. `$Polyad{{poly}{}}` — the
	/// self-referential polygon — has two slots, so it crosses as two points: no
	/// polygon, no fill, a field left dark. Its `poly` slot is never read, never
	/// dereferenced, never called upon — it is met here only as the number two.
	#[test]
	fn the_self_referential_polyad_lights_nothing_and_is_never_called_upon() {
		use crate::game::poly::Poly;
		let polys = Poly::load("res/entities/poly.poly").expect("load poly.poly");
		let polyad = polys.iter().find(|p| p.name == "$Polyad").expect("the file holds $Polyad");

		let mut field = LightField::new(20, 20);
		field.fill_polygon(&regular_polygon(polyad.slots.len(), 10.0, 10.0, 6.0, 0.0), 1.0);
		assert_eq!(field.max(), 0.0, "a degenerate cyclic deposits no light");
	}

	#[test]
	fn a_wireframe_lights_its_edges_and_leaves_the_interior_dark() {
		let mut field = LightField::new(20, 20);
		// A diamond (four points on the circle); its rim is lit, its middle is not.
		field.outline_polygon(&regular_polygon(4, 10.0, 10.0, 6.0, 0.0), 1.0);
		assert!(field.get(10, 4) > 0.0, "a vertex on the rim is lit");
		assert_eq!(field.get(10, 10), 0.0, "the interior stays dark — a wireframe, not a fill");
	}
}
