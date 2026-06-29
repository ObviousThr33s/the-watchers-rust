//! Items — things that lie in the world to be picked up, carried, and set down
//! again. What an item *is* (its name, its glyph) is data; the carrying is
//! mechanism. One item and a pocket to hold it is the whole of it for now.

use crate::game::entity::EntityId;

/// A thing that can be carried: what it is (`name`), how it shows (`glyph`), and
/// the id it answers to while it lies in the field. Picking it up takes it out of
/// the field and into a pocket; dropping it sets it back down under the same id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
	pub id: EntityId,
	pub glyph: char,
	pub name: String,
}
