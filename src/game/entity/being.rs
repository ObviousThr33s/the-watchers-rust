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
}

impl Being {
	/// Parses one `.being` definition: `key value` header lines, then a line
	/// that is exactly `---`, then the ascii art verbatim to the end. Blank
	/// lines and `#` comments in the header are ignored.
	pub fn parse(text: &str) -> Result<Being, String> {
		let mut being = Being { glyph: '?', visible: true, ..Being::default() };
		let mut lines = text.lines();
		let mut saw_separator = false;

		for line in lines.by_ref() {
			if line.trim() == "---" {
				saw_separator = true;
				break;
			}
			let trimmed = line.trim();
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

		// Everything past the separator is art, verbatim.
		if saw_separator {
			let rest: Vec<&str> = lines.collect();
			being.art = rest.join("\n");
		}

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_header_and_art() {
		let text = "name Oolooroo\nglyph F\nvisible true\nhealth 10\npower 10\nbloom 2\nbehavior gentle\n---\nAAA\nBBB";
		let b = Being::parse(text).expect("parse");
		assert_eq!(b.name, "Oolooroo");
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
	fn loads_oolooroo_from_disk() {
		let b = Being::load("res/entities/oolooroo.being").expect("load oolooroo");
		assert_eq!(b.name, "Oolooroo");
		assert_eq!(b.glyph, 'F');
		assert_eq!(b.behavior, "gentle");
		assert!(!b.art.is_empty());
	}
}
