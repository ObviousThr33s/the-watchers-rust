//! Entity *definitions* — "the entity with the data," kept distinct from the
//! in-world "entity with the ID" (`Entity`). One self-contained `.being` file
//! holds a kind's text, its numbers, and its art, so adding a new watcher is
//! adding data, not code. Parsing is a pure function over a string; the file
//! wrapper on top is thin, and a malformed file is an error, never a panic.

/// What can currently be seen of a thing. `Partial` is the seam where failing
/// light, signal corruption, and half-remembered places all live: `0.0` is
/// nearly gone, `1.0` is nearly clear.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Visibility {
	Visible,
	Hidden,
	Partial(f32),
}

/// The data half of an entity: simple text, simple numbers, and ascii art —
/// loaded from one `.being` definition.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Being {
	pub name: String,
	pub glyph: char,
	pub visible: bool,
	pub health: i32,
	pub power: i32,
	/// Glyphs revealed per tick as the art blooms into view.
	pub bloom: u16,
	/// Behaviour tag — "gentle" is a friend, the rest are watchers.
	pub behavior: String,
	pub art: String,
	/// The narrative line this being surfaces when seen. Authored in the file,
	/// never in code — the story lives at the file boundary, not the engine.
	pub line: String,
}

impl Being {
	/// Parses one `.being` definition: `key value` header lines, then a line
	/// that is exactly `---`, then the ascii art verbatim to the end. Blank
	/// lines and `#` comments in the header are ignored.
	pub fn parse(text: &str) -> Result<Being, String> {
		let mut being = Being { glyph: '?', visible: true, ..Being::default() };
		let mut lines = text.lines();
		let mut block: Option<String> = None;

		// Header: `key value` lines until the first `---` separator. A separator
		// may name the block it opens (`--- line`); a bare `---` opens `art`.
		for line in lines.by_ref() {
			let trimmed = line.trim();
			if let Some(rest) = trimmed.strip_prefix("---") {
				let name = rest.trim();
				block = Some(if name.is_empty() { "art".into() } else { name.into() });
				break;
			}
			if trimmed.is_empty() || trimmed.starts_with('#') {
				continue;
			}

			let (key, value) = trimmed
				.split_once(char::is_whitespace)
				.ok_or_else(|| format!("not a `key value` line: {line:?}"))?;
			let value = value.trim();

			match key {
				"name" => being.name = value.to_string(),
				"glyph" => being.glyph = value.chars().next().ok_or("glyph is empty")?,
				"visible" => being.visible = value == "true",
				"health" => being.health = value.parse().map_err(|_| format!("health not a number: {value:?}"))?,
				"power" => being.power = value.parse().map_err(|_| format!("power not a number: {value:?}"))?,
				"bloom" => being.bloom = value.parse().map_err(|_| format!("bloom not a number: {value:?}"))?,
				"behavior" => being.behavior = value.to_string(),
				other => return Err(format!("unknown key: {other:?}")),
			}
		}

		// Verbatim blocks: each block runs until the next `---` line or the end
		// of the file, kept exactly. Unknown block names are ignored, so a file
		// can carry narrative sections before the engine reads them — the story
		// can grow ahead of the code.
		let mut name = match block {
			Some(name) => name,
			None => return Ok(being), // header-only file: no art, no narrative
		};
		let mut buf: Vec<&str> = Vec::new();
		for line in lines.by_ref() {
			if let Some(rest) = line.trim().strip_prefix("---") {
				assign_block(&mut being, &name, buf.join("\n"));
				buf.clear();
				let next = rest.trim();
				name = if next.is_empty() { "art".into() } else { next.into() };
				continue;
			}
			buf.push(line);
		}
		assign_block(&mut being, &name, buf.join("\n"));

		Ok(being)
	}

	/// Loads and parses a `.being` file (UTF-8). A missing or malformed file is
	/// a descriptive error, never a panic.
	pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Being, String> {
		let text = std::fs::read_to_string(&path)
			.map_err(|e| format!("could not read {}: {e}", path.as_ref().display()))?;
		Being::parse(&text)
	}
}

/// Routes a verbatim block's content into the matching `Being` field. Unknown
/// block names are dropped, so a file can carry sections the engine does not
/// read yet — the story can grow ahead of the code.
fn assign_block(being: &mut Being, name: &str, content: String) {
	match name {
		"art" => being.art = content,
		"line" => being.line = content,
		_ => {}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_header_and_art() {
		let text = "name Ooloonoo\nglyph F\nvisible true\nhealth 10\npower 10\nbloom 2\nbehavior gentle\n---\nAAA\nBBB";
		let b = Being::parse(text).expect("parse");
		assert_eq!(b.name, "Ooloonoo");
		assert_eq!(b.glyph, 'F');
		assert!(b.visible);
		assert_eq!(b.health, 10);
		assert_eq!(b.power, 10);
		assert_eq!(b.bloom, 2);
		assert_eq!(b.behavior, "gentle");
		assert_eq!(b.art, "AAA\nBBB");
	}

	#[test]
	fn ignores_comments_and_blank_lines() {
		let b = Being::parse("# a watcher\n\nname Wisp\nglyph w\n---\nart").unwrap();
		assert_eq!(b.name, "Wisp");
		assert_eq!(b.glyph, 'w');
		assert_eq!(b.art, "art");
	}

	#[test]
	fn missing_art_section_is_fine() {
		let b = Being::parse("name Bare\nglyph b").unwrap();
		assert_eq!(b.name, "Bare");
		assert_eq!(b.art, "");
	}

	#[test]
	fn unknown_key_is_an_error_not_a_panic() {
		assert!(Being::parse("wat huh\n---\n").is_err());
	}

	#[test]
	fn defaults_when_unspecified() {
		let b = Being::parse("name Nameless").unwrap();
		assert_eq!(b.glyph, '?'); // shown rather than crashing
		assert!(b.visible);
		assert_eq!(b.health, 0);
	}

	#[test]
	fn loads_ooloonoo_from_disk() {
		let b = Being::load("res/entities/ooloonoo.being").expect("load ooloonoo");
		assert_eq!(b.name, "Ooloonoo");
		assert_eq!(b.glyph, 'F');
		assert_eq!(b.behavior, "gentle");
		assert!(!b.art.is_empty());
		assert!(!b.line.is_empty(), "Ooloonoo should surface a narrative line from her file");
	}

	#[test]
	fn loads_oolooroo_the_twin_from_disk() {
		let b = Being::load("res/entities/oolooroo.being").expect("load oolooroo");
		assert_eq!(b.name, "Oolooroo");
		assert_eq!(b.glyph, 'F'); // shares the fairy glyph with Ooloonoo — a twin you can mistake
		assert_eq!(b.behavior, "gentle");
		assert!(!b.art.is_empty());
		assert!(!b.line.is_empty());
	}

	#[test]
	fn loads_vesh_the_watcher_from_disk() {
		// Vesh carries `--- faction` and `--- behaviors` blocks the engine does
		// not read yet; the file must still parse cleanly (unknown blocks are
		// dropped) — the design grows ahead of the code.
		let b = Being::load("res/entities/vesh.being").expect("load vesh");
		assert_eq!(b.name, "Vesh");
		assert_eq!(b.glyph, 'v');
		assert!(!b.art.is_empty());
		assert!(!b.line.is_empty());
	}

	#[test]
	fn parses_named_art_and_line_blocks() {
		let text = "name Ooloonoo\nglyph F\n--- art\nAAA\nBBB\n--- line\nshe waits, unblinking";
		let b = Being::parse(text).expect("parse");
		assert_eq!(b.art, "AAA\nBBB");
		assert_eq!(b.line, "she waits, unblinking");
	}

	#[test]
	fn unknown_block_is_ignored_for_forward_compat() {
		let b = Being::parse("name X\n--- art\nart\n--- future\nnot read yet").unwrap();
		assert_eq!(b.art, "art");
		assert_eq!(b.line, "");
	}
}
