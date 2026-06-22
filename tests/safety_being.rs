//! Absolute-safety tests for the `.being` parser — its hostile-input contract.
//!
//! `.being` files are *content*, authored by anyone in a public, collaborative
//! repo. The promise this crate guards: no file can make `Being::parse` panic,
//! overflow, or misread bytes. Every input resolves to `Ok` or a typed `Err`.
//!
//! These are integration tests (ch. 11.3): one crate, public API only. Unit
//! tests for the parser's internals live inline in `src/game/entity/being.rs`.
//! Each test below is a sentence about a guarantee; a red one means a safety
//! property the engine leans on has been lost.

use obelisk::game::entity::being::{Being, BeingError};

/// A number too large for its field comes back as a typed error, not a silent
/// wraparound — `i32::from_str` rejects the overflow before it can wrap.
#[test]
fn an_oversized_numeric_field_is_a_typed_error_not_an_overflow() {
	let huge = "name X\nhealth 99999999999999999999999999";
	match Being::parse(huge) {
		Err(BeingError::NotANumber { key, .. }) => assert_eq!(key, "health"),
		other => panic!("expected a typed NotANumber error, got {other:?}"),
	}
}

/// `bloom` is a `u16`; a negative or out-of-range value is rejected, never
/// truncated into a different number.
#[test]
fn an_out_of_range_unsigned_field_is_rejected_not_truncated() {
	assert!(matches!(
		Being::parse("name X\nbloom -1"),
		Err(BeingError::NotANumber { .. })
	));
	assert!(matches!(
		Being::parse("name X\nbloom 70000"),
		Err(BeingError::NotANumber { .. })
	));
}

/// A glyph is taken as one *character*, not one byte — a multibyte scalar must
/// not be sliced mid-codepoint (which would panic).
#[test]
fn a_multibyte_glyph_is_read_whole_not_sliced() {
	let being = Being::parse("name X\nglyph 世").expect("multibyte glyph parses");
	assert_eq!(being.glyph, '世');
}

/// Pathologically large input parses without panicking or stack-blowing.
#[test]
fn a_very_large_file_does_not_panic() {
	let mut text = String::from("name Huge\n---\n");
	text.push_str(&"A".repeat(200_000));
	let being = Being::parse(&text).expect("a large file still parses");
	assert_eq!(being.art.len(), 200_000);
}

/// Thousands of block separators are arrival data, not an error or a crash.
#[test]
fn a_storm_of_separators_does_not_panic() {
	let text = format!("name X\n{}", "---\n".repeat(5_000));
	assert!(Being::parse(&text).is_ok());
}

/// Windows line endings (`\r\n`) parse cleanly — the trailing `\r` is trimmed,
/// not carried into a value or a key.
#[test]
fn crlf_line_endings_parse_without_carrying_the_carriage_return() {
	let being = Being::parse("name Wisp\r\nglyph w\r\n").expect("crlf parses");
	assert_eq!(being.name, "Wisp");
	assert_eq!(being.glyph, 'w');
}

/// A header line that is only whitespace is skipped, not treated as a malformed
/// `key value` pair.
#[test]
fn a_whitespace_only_header_line_is_skipped_not_an_error() {
	let being = Being::parse("name X\n   \nglyph y").expect("blank-ish line is fine");
	assert_eq!(being.glyph, 'y');
}

/// Control bytes (including NUL) inside an art block are preserved verbatim, not
/// a parse failure or a panic — the art is kept exactly.
#[test]
fn control_bytes_in_an_art_block_are_preserved_not_panicked() {
	let being = Being::parse("name X\n---\na\0b\x07c").expect("control bytes parse");
	assert_eq!(being.art, "a\0b\x07c");
}

/// A header key with no value is a typed error, never an index panic.
#[test]
fn a_key_with_no_value_is_a_typed_error() {
	assert!(matches!(
		Being::parse("nameonly"),
		Err(BeingError::NotKeyValue(_))
	));
}

/// Empty input is the default being, not a crash — the floor case is safe.
#[test]
fn empty_input_yields_a_default_being() {
	let being = Being::parse("").expect("empty input is a default being");
	assert_eq!(being, Being { glyph: '?', visible: true, ..Being::default() });
}
