//! Polygon *definitions* — the node structure the roadmap's "circular nodes with
//! contact points" finally names in code. A `.poly` body is a named shape whose
//! slots are its vertices: each slot touches another polygon, carries a token, or
//! is empty. Parsing is a pure function over a string; the grammar lives here, the
//! polygons themselves live in `.poly` files (content, never hardcoded story).

//this class is a group effort.
//this .rs is also a group effort.
//language can be limiting.
//this is our quantum leap
//dev ops is working on many solutions.
//doing our own is hard.
//how handle?
//also. where to find polygon.
//math

use std::fmt;

/// One vertex of a polygon — a contact point. Either empty (`{}`) or filled with a
/// name/token (`{Tetrad}`, `{adergo}`). Whether a filled slot names a sub-polygon
/// or a bare token is meaning the engine reads later; the parser keeps only the
/// faithful structure, so the design can grow ahead of the code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Slot {
	Empty,
	Filled(String),
}

/// A polygon: a named shape (`Pentad`) and its ordered slots — its vertices. An
/// anonymous body (`{{a}{b}}` with no leading name) parses with an empty `name`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Poly {
	pub name: String,
	pub slots: Vec<Slot>,
}

/// Why a `.poly` body failed to parse. Typed rather than a free string so a caller
/// can match the cause — a malformed file is always an error, never a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolyError {
	/// Nothing but whitespace to parse.
	Empty,
	/// A name with no `{ ... }` body after it.
	NoBody { name: String },
	/// Braces don't balance — a `{` with no matching `}`.
	Unbalanced,
	/// Stray text where a `{ ... }` slot was expected.
	UnexpectedText(String),
	/// The file could not be read from disk.
	Io { path: String, message: String },
}

impl fmt::Display for PolyError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PolyError::Empty => write!(f, "nothing to parse"),
			PolyError::NoBody { name } => write!(f, "{name:?} has no body"),
			PolyError::Unbalanced => write!(f, "unbalanced braces"),
			PolyError::UnexpectedText(t) => write!(f, "unexpected text where a slot was expected: {t:?}"),
			PolyError::Io { path, message } => write!(f, "could not read {path}: {message}"),
		}
	}
}

impl std::error::Error for PolyError {}

impl Poly {
	/// Parses one polygon body: an optional name, then a brace group whose direct
	/// children are the slots. `Pentad{{Tetrad}{adergo}{}{}{}}` is a `Pentad` with
	/// five slots — a nested `Tetrad`, a token, and three empty.
	///
	/// ```
	/// use obelisk::game::poly::{Poly, Slot};
	///
	/// let p = Poly::parse("Triad{{a}{b}{}}").unwrap();
	/// assert_eq!(p.name, "Triad");
	/// assert_eq!(p.slots.len(), 3);
	/// assert!(p.slots.contains(&Slot::Empty));
	/// ```
	pub fn parse(text: &str) -> Result<Poly, PolyError> {
		let text = text.trim();
		if text.is_empty() {
			return Err(PolyError::Empty);
		}

		// The name is everything before the first '{'; the body is the balanced
		// group that opens there and must run to the end of the input.
		let open = text
			.find('{')
			.ok_or_else(|| PolyError::NoBody { name: text.to_string() })?;
		let name = text.get(..open).unwrap_or("").trim().to_string();
		let (inner, rest) = balanced(text.get(open..).unwrap_or(""))?;
		if !rest.trim().is_empty() {
			return Err(PolyError::UnexpectedText(rest.trim().to_string()));
		}

		// The body's direct children are the slots: each is itself a `{ ... }` group,
		// empty or filled. Brace depth carries nested groups through whole.
		let mut slots = Vec::new();
		let mut cursor = inner.trim_start();
		while !cursor.is_empty() {
			if !cursor.starts_with('{') {
				return Err(PolyError::UnexpectedText(cursor.to_string()));
			}
			let (content, after) = balanced(cursor)?;
			let content = content.trim();
			slots.push(if content.is_empty() {
				Slot::Empty
			} else {
				Slot::Filled(content.to_string())
			});
			cursor = after.trim_start();
		}

		Ok(Poly { name, slots })
	}

	/// Reads every polygon definition in a `.poly` body, in file order. A line
	/// that is blank or starts with `#` is a comment — a label for the reader,
	/// dropped here (the story can grow ahead of the code). Each remaining
	/// definition is an optional name then one balanced `{ ... }` group, handed
	/// to `parse` so slot logic lives in one place. Faithful only: a `Filled`
	/// slot that names another polygon stays a name. This reads the file in; it
	/// never resolves a reference.
	pub fn parse_all(text: &str) -> Result<Vec<Poly>, PolyError> {
		// Drop comment lines first, keeping the rest verbatim so a brace group
		// that spans lines survives intact.
		let body: String = text
			.lines()
			.filter(|line| !line.trim_start().starts_with('#'))
			.collect::<Vec<_>>()
			.join("\n");

		let mut polys = Vec::new();
		let mut cursor = body.trim();
		while !cursor.is_empty() {
			// One definition reaches from here to the end of its balanced group.
			let open = cursor
				.find('{')
				.ok_or_else(|| PolyError::NoBody { name: cursor.to_string() })?;
			let name = cursor.get(..open).unwrap_or("").trim();
			let (inner, rest) = balanced(cursor.get(open..).unwrap_or(""))?;
			let end = cursor.len() - rest.len();

			// A bare-inner anonymous group like `{Triad}` is a lone *slot* — a
			// reference, the cyclic the file illustrates — not a polygon body
			// (whose inner is itself `{ ... }` slots). It is met in the mirror and
			// passed over: never parsed as a shape, never called upon. Only true
			// polygon bodies are loaded.
			let inner = inner.trim();
			if name.is_empty() && !inner.is_empty() && !inner.starts_with('{') {
				cursor = rest.trim();
				continue;
			}

			polys.push(Poly::parse(cursor.get(..end).unwrap_or("").trim())?);
			cursor = rest.trim();
		}

		if polys.is_empty() {
			return Err(PolyError::Empty);
		}
		Ok(polys)
	}

	/// Loads and parses every polygon in a `.poly` file (UTF-8). A missing or
	/// malformed file is a descriptive error, never a panic. Read-and-parse only:
	/// the loader meets even a self-referential polygon in the mirror — it becomes
	/// a named slot — and never calls upon it.
	pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Poly>, PolyError> {
		let text = std::fs::read_to_string(&path).map_err(|e| PolyError::Io {
			path: path.as_ref().display().to_string(),
			message: e.to_string(),
		})?;
		Poly::parse_all(&text)
	}
}

/// Reads one balanced `{ ... }` group from the front of `s`, returning the text
/// *inside* the outer braces and whatever follows the closing brace. `s` must
/// start with `{`; depth-tracking lets nested groups pass through intact.
fn balanced(s: &str) -> Result<(&str, &str), PolyError> {
	let bytes = s.as_bytes();
	if bytes.first() != Some(&b'{') {
		return Err(PolyError::Unbalanced);
	}
	let mut depth: u32 = 0;
	for (i, &b) in bytes.iter().enumerate() {
		match b {
			b'{' => depth += 1,
			b'}' => {
				depth = depth.checked_sub(1).ok_or(PolyError::Unbalanced)?;
				if depth == 0 {
					let inner = s.get(1..i).ok_or(PolyError::Unbalanced)?;
					let rest = s.get(i + 1..).unwrap_or("");
					return Ok((inner, rest));
				}
			}
			_ => {}
		}
	}
	Err(PolyError::Unbalanced)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// The finish line — and no longer the placebo. This reads the real
	/// `res/entities/poly.poly` and finds the `Pentad` in the loaded collection:
	/// five slots — a nested `Tetrad`, the token `adergo`, then three empty,
	/// because a pentagon has five vertices and each vertex is a contact point.
	#[test]
	fn a_pentad_has_five_slots_read_from_the_poly_file() {
		let polys = Poly::load("res/entities/poly.poly").expect("load poly.poly");
		let p = polys
			.iter()
			.find(|p| p.name == "Pentad")
			.expect("the file holds a Pentad");
		assert_eq!(p.slots.len(), 5, "a pentad has five vertices");
		assert_eq!(
			p.slots,
			vec![
				Slot::Filled("Tetrad".into()),
				Slot::Filled("adergo".into()),
				Slot::Empty,
				Slot::Empty,
				Slot::Empty,
			],
		);
	}

	/// Good faith, twice over. Every *polygon* in the file is read — the anon
	/// triad, the Pentad, the self-referential `$Polyad` — three in all. The lone
	/// `{Triad}` under `#cyclic` is a *slot*, not a polygon: met in the mirror and
	/// passed over (see `parse_all`), never resolved. And `$Polyad` keeps its
	/// `poly` slot as a name, never dereferenced into itself.
	#[test]
	fn load_leaves_no_one_behind_and_never_calls_upon_the_cyclic() {
		let polys = Poly::load("res/entities/poly.poly").expect("load poly.poly");
		assert_eq!(polys.len(), 3, "every polygon body is read; the cyclic slot is not one");

		let polyad = polys
			.iter()
			.find(|p| p.name == "$Polyad")
			.expect("the file holds the self-referential Polyad");
		assert_eq!(
			polyad.slots,
			vec![Slot::Filled("poly".into()), Slot::Empty],
			"the cyclic stays a name in the mirror — never dereferenced",
		);

		// Show the working: the roster read straight from disk (see with --nocapture).
		eprintln!("--- polygons read from res/entities/poly.poly ---");
		for p in &polys {
			let name = if p.name.is_empty() { "(anon)" } else { p.name.as_str() };
			eprintln!("  {name}: {:?}", p.slots);
		}
		eprintln!("  (passed over: {{Triad}} — the cyclic slot, met in the mirror, never called upon)");
	}
}
