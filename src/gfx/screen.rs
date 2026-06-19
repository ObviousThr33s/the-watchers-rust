//! The screen buffer, built like a shōji: panels of translucent paper over a
//! frame. A panel is a grid of [`Cell`]s; painting is done by coordinate, not by
//! pushing glyphs in sequence. Panels stack — [`Screen::compose`] lays one over
//! another, and a cell's [`Ink`] decides how much of the layer beneath shows
//! through. Empty paper (negative space — *ma*) is as deliberate as ink, so an
//! unpainted cell reveals what is behind it rather than blanking it out.
//!
//! A panel can also be veiled and unveiled over time (see [`Screen::reveal_to`]):
//! the substrate for paced, story-driven reveal, where a scene or a line of text
//! is drawn in stages like a sliding panel opening.

use ratatui::{
	style::{Color, Modifier, Style},
	text::{Line, Span, Text},
};

/// How much of the layer beneath a cell shows through when panels are composed.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Ink {
	/// Bare paper. Nothing is painted here; the layer behind shows through
	/// untouched. This is the default — a fresh panel is all negative space.
	#[default]
	Empty,
	/// A wash. The glyph behind is kept, but this cell's colour and attributes
	/// are laid over it — a translucent pane that tints rather than hides.
	/// Fog, mist, dusk, and dimming are washes.
	Wash,
	/// Solid ink. Hides whatever is behind it completely.
	Opaque,
}

/// One cell of a panel: a glyph and how it is painted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
	pub glyph: char,
	pub fg: Color,
	pub bg: Color,
	pub modifier: Modifier,
	pub ink: Ink,
}

impl Default for Cell {
	/// Blank paper: a space, no colour, transparent to the layer behind.
	fn default() -> Self {
		Cell {
			glyph: ' ',
			fg: Color::Reset,
			bg: Color::Reset,
			modifier: Modifier::empty(),
			ink: Ink::Empty,
		}
	}
}

impl Cell {
	/// An opaque glyph in the terminal's default colours.
	pub fn glyph(glyph: char) -> Self {
		Cell { glyph, ink: Ink::Opaque, ..Cell::default() }
	}

	/// An opaque glyph in a chosen foreground colour.
	pub fn inked(glyph: char, fg: Color) -> Self {
		Cell { glyph, fg, ink: Ink::Opaque, ..Cell::default() }
	}

	/// A translucent wash: the glyph paints, but composed over another panel it
	/// tints the cell behind instead of replacing it. Used for fog and mist.
	pub fn wash(glyph: char, fg: Color, modifier: Modifier) -> Self {
		Cell { glyph, fg, modifier, ink: Ink::Wash, ..Cell::default() }
	}

	fn style(&self) -> Style {
		Style::default().fg(self.fg).bg(self.bg).add_modifier(self.modifier)
	}
}

/// A panel of the shōji: a `x` by `y` grid of cells, drawn into by coordinate.
#[derive(Clone)]
pub struct Screen {
	/// Width in cells. Named `x`/`y` for the grid axes a caller addresses.
	pub x: u16,
	pub y: u16,
	cells: Vec<Cell>,
	/// Glyph shown for a cell that is currently veiled (masked off).
	veil: char,
	/// Per-cell reveal mask. `None` means the whole panel is shown; `Some`
	/// shows only the `true` cells and veils the rest — paced reveal.
	mask: Option<Vec<bool>>,
}

impl Screen {
	pub fn new(width: u16, height: u16) -> Self {
		Screen {
			x: width,
			y: height,
			cells: vec![Cell::default(); width as usize * height as usize],
			veil: '░',
			mask: None,
		}
	}

	pub fn width(&self) -> u16 { self.x }
	pub fn height(&self) -> u16 { self.y }

	/// Row-major index of `(x, y)`, or `None` if it lies off the panel.
	fn index(&self, x: u16, y: u16) -> Option<usize> {
		if x < self.x && y < self.y {
			Some(y as usize * self.x as usize + x as usize)
		} else {
			None
		}
	}

	/// The cell at `(x, y)`, if on the panel.
	pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
		self.index(x, y).map(|i| &self.cells[i])
	}

	/// Paint `cell` at `(x, y)`. Off-panel coordinates are ignored, so callers
	/// never have to bounds-check before drawing.
	pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
		if let Some(i) = self.index(x, y) {
			self.cells[i] = cell;
		}
	}

	/// Paint an opaque `glyph` at `(x, y)` in the default colours.
	pub fn put(&mut self, x: u16, y: u16, glyph: char) {
		self.set(x, y, Cell::glyph(glyph));
	}

	/// Reset every cell to blank paper and lift any veil.
	pub fn clear(&mut self) {
		for cell in &mut self.cells {
			*cell = Cell::default();
		}
		self.mask = None;
	}

	/// Fill the whole panel with one cell.
	pub fn fill(&mut self, cell: Cell) {
		for c in &mut self.cells {
			*c = cell;
		}
	}

	/// The glyph shown for veiled cells (default `'░'`).
	pub fn set_veil(&mut self, veil: char) {
		self.veil = veil;
	}

	/// Lay `top` over this panel. For each cell, [`Ink`] decides the result:
	/// `Empty` keeps what is here, `Opaque` replaces it, and `Wash` keeps this
	/// glyph but lays the top cell's colour and attributes over it. Panels must
	/// share dimensions; a mismatch is a no-op.
	pub fn compose(&mut self, top: &Screen) {
		if self.x != top.x || self.y != top.y {
			return;
		}
		for (base, over) in self.cells.iter_mut().zip(top.cells.iter()) {
			match over.ink {
				Ink::Empty => {}
				Ink::Opaque => *base = *over,
				Ink::Wash => {
					// A tint: keep the glyph behind, lay the wash's colour over.
					base.modifier |= over.modifier;
					if over.fg != Color::Reset {
						base.fg = over.fg;
					}
					if over.bg != Color::Reset {
						base.bg = over.bg;
					}
					base.ink = Ink::Opaque;
				}
			}
		}
	}

	// --- Paced reveal -------------------------------------------------------
	//
	// A veiled panel shows only the cells turned on in its mask; everything
	// else reads as `veil`. The first reveal call on an unveiled panel conceals
	// it first, so "reveal these" means "show only these".

	fn ensure_mask(&mut self) -> &mut Vec<bool> {
		if self.mask.is_none() {
			self.mask = Some(vec![false; self.cells.len()]);
		}
		self.mask.as_mut().unwrap()
	}

	/// Veil the whole panel — nothing shows until revealed.
	pub fn conceal_all(&mut self) {
		self.mask = Some(vec![false; self.cells.len()]);
	}

	/// Lift the veil entirely.
	pub fn reveal_all(&mut self) {
		self.mask = None;
	}

	/// Reveal a single cell.
	pub fn reveal(&mut self, x: u16, y: u16) {
		if let Some(i) = self.index(x, y) {
			self.ensure_mask()[i] = true;
		}
	}

	/// Reveal a rectangle of cells, clamped to the panel.
	pub fn reveal_rect(&mut self, x: u16, y: u16, w: u16, h: u16) {
		for yy in y..y.saturating_add(h) {
			for xx in x..x.saturating_add(w) {
				self.reveal(xx, yy);
			}
		}
	}

	/// Reveal the first `n` cells in reading order (left to right, top to
	/// bottom) and veil the rest — a typewriter sweep for paced text.
	pub fn reveal_to(&mut self, n: usize) {
		let total = self.cells.len();
		let mut mask = vec![false; total];
		for slot in mask.iter_mut().take(n.min(total)) {
			*slot = true;
		}
		self.mask = Some(mask);
	}

	/// Whether `(x, y)` is currently shown. Off-panel cells are not shown.
	pub fn is_revealed(&self, x: u16, y: u16) -> bool {
		match self.index(x, y) {
			Some(i) => self.mask.as_ref().map_or(true, |m| m[i]),
			None => false,
		}
	}

	/// How many cells are currently shown.
	pub fn revealed_count(&self) -> usize {
		match &self.mask {
			None => self.cells.len(),
			Some(m) => m.iter().filter(|&&v| v).count(),
		}
	}

	fn shown(&self, i: usize) -> bool {
		self.mask.as_ref().map_or(true, |m| m[i])
	}

	// --- Read-out -----------------------------------------------------------

	/// The panel's glyphs as plain text, rows joined by newlines. Veiled cells
	/// read as the veil glyph. Colour is dropped — see [`Screen::to_text`].
	pub fn to_string(&self) -> String {
		let mut out = String::with_capacity((self.x as usize + 1) * self.y as usize);
		for row in 0..self.y {
			if row > 0 {
				out.push('\n');
			}
			for col in 0..self.x {
				let i = row as usize * self.x as usize + col as usize;
				out.push(if self.shown(i) { self.cells[i].glyph } else { self.veil });
			}
		}
		out
	}

	/// The panel as styled [`Text`], one [`Line`] per row, with runs of the same
	/// style coalesced into a single [`Span`]. Veiled cells render as a dim veil.
	pub fn to_text(&self) -> Text<'static> {
		let veil_style = Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM);
		let mut lines: Vec<Line> = Vec::with_capacity(self.y as usize);

		for row in 0..self.y {
			let mut spans: Vec<Span> = Vec::new();
			let mut run = String::new();
			let mut run_style = Style::default();

			for col in 0..self.x {
				let i = row as usize * self.x as usize + col as usize;
				let (glyph, style) = if self.shown(i) {
					(self.cells[i].glyph, self.cells[i].style())
				} else {
					(self.veil, veil_style)
				};

				if !run.is_empty() && style != run_style {
					spans.push(Span::styled(std::mem::take(&mut run), run_style));
				}
				run_style = style;
				run.push(glyph);
			}
			if !run.is_empty() {
				spans.push(Span::styled(run, run_style));
			}
			lines.push(Line::from(spans));
		}

		Text::from(lines)
	}
}
//remember what I said about tests being a good way to inform documentation.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn addressing_is_bounds_safe() {
		let mut s = Screen::new(3, 2);
		s.put(1, 1, 'A');
		s.put(9, 9, 'B'); // off-panel: silently ignored, no panic
		assert_eq!(s.get(1, 1).unwrap().glyph, 'A');
		assert!(s.get(9, 9).is_none());
		assert_eq!(s.to_string(), "   \n A ");
	}

	#[test]
	fn empty_cells_let_the_layer_behind_show_through() {
		// Base has a glyph; top is bare paper over it -> base survives.
		let mut base = Screen::new(2, 1);
		base.put(0, 0, '#');
		base.put(1, 0, '#');
		let top = Screen::new(2, 1); // all Empty
		base.compose(&top);
		assert_eq!(base.to_string(), "##");
	}

	#[test]
	fn opaque_ink_hides_what_is_behind() {
		let mut base = Screen::new(2, 1);
		base.fill(Cell::glyph('#'));
		let mut top = Screen::new(2, 1);
		top.put(1, 0, '@'); // opaque
		base.compose(&top);
		assert_eq!(base.to_string(), "#@");
	}

	#[test]
	fn a_wash_tints_the_glyph_behind_without_replacing_it() {
		let mut base = Screen::new(1, 1);
		base.set(0, 0, Cell::inked('#', Color::White));
		let mut fog = Screen::new(1, 1);
		fog.set(0, 0, Cell::wash('░', Color::DarkGray, Modifier::DIM));
		base.compose(&fog);
		let cell = base.get(0, 0).unwrap();
		assert_eq!(cell.glyph, '#', "the wash keeps the glyph behind it");
		assert_eq!(cell.fg, Color::DarkGray, "but takes on the wash's colour");
		assert!(cell.modifier.contains(Modifier::DIM));
	}

	#[test]
	fn reveal_in_reading_order_unveils_a_prefix() {
		let mut s = Screen::new(3, 1);
		s.put(0, 0, 'a');
		s.put(1, 0, 'b');
		s.put(2, 0, 'c');
		s.reveal_to(2);
		assert_eq!(s.to_string(), "ab░");
		assert_eq!(s.revealed_count(), 2);
		s.reveal_all();
		assert_eq!(s.to_string(), "abc");
	}

	#[test]
	fn concealing_then_revealing_shows_only_what_is_chosen() {
		let mut s = Screen::new(3, 1);
		s.fill(Cell::glyph('x'));
		s.conceal_all();
		assert_eq!(s.to_string(), "░░░");
		s.reveal(1, 0);
		assert_eq!(s.to_string(), "░x░");
		assert!(s.is_revealed(1, 0));
		assert!(!s.is_revealed(0, 0));
	}

	#[test]
	fn styled_text_has_one_line_per_row() {
		let mut s = Screen::new(2, 2);
		s.set(0, 0, Cell::inked('A', Color::Red));
		let text = s.to_text();
		assert_eq!(text.lines.len(), 2);
	}
}
