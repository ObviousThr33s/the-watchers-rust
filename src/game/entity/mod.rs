//! The in-world half of an entity: "the entity with the ID." Where a
//! [`being::Being`] is the *data* a kind is made of (text, numbers, art loaded
//! from a `.being` file), [`Entity`] is the thing actually standing in a
//! [`Field`](crate::game::spaces::field::Field) — a glyph at a position, with an
//! id and a draw priority. One is the definition; the other is the instance.

use std::fmt;

use actor::Actor;

pub mod player;
pub mod actor;
pub mod sekaikan;

pub mod being;

/// A unique handle to an entity — the identity the event bus moves (ward 1, see
/// `CLAUDE.md`) and the key the [`Field`](crate::game::spaces::field::Field) and
/// every lookup use. A plain `u64` so an [`Event`](crate::game::haps::Event) stays
/// `Copy`, heap-free, and trivially queueable; the *readable* name a thing carries
/// lives in its [`being::Being`] data, looked up by id, never stored here.
pub type EntityId = u64;

/// The player's well-known id. The player is the one hardcoded spawn (see
/// [`player`]), so it gets a fixed id every system can name without a lookup;
/// minted ids start at `1` (see [`Field::mint`](crate::game::spaces::field::Field::mint)),
/// so they never collide with it.
pub const PLAYER: EntityId = 0;

/// A thing standing in the field: a glyph at a grid position, addressed by a
/// unique [`EntityId`]. Everything a kind is *made of* (stats, art, the line it
/// surfaces when seen) lives in its [`being::Being`]; this holds only what the
/// world needs to place it, draw it, and find it again.
#[derive(Clone)]
pub struct Entity {
	pub x: i16,
	pub y: i16,
	/// Who wins a shared cell when the map is rasterized — higher draws over lower.
	pub priority: Priority,
	/// The glyph drawn for this entity. (`self` is a keyword, hence the trailing `_`.)
	pub self_: char,
	/// Unique key into the [`Field`](crate::game::spaces::field::Field) — also how
	/// the spatial index and every lookup find it.
	pub id: EntityId,
}

/// Draw priority for a shared cell: the higher entity is the one painted. Floor
/// scenery (walls, flora) sits at `LOW`, with the player and beings above it.
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Priority {
	LOW = 0,
	MED = 1,
	HIG = 2,
}

impl Entity {

	/// Build an entity at `(x, y)`, drawn as `self_` and keyed by `id`.
	pub fn new(x: i16, y: i16, self_: char, id:EntityId, priority:Priority) -> Self {
		Entity { x, y, self_, id, priority}
	}

	/// Set the grid position directly. This does *not* touch any field's spatial
	/// index — go through [`Field::move_entity`](crate::game::spaces::field::Field::move_entity)
	/// or [`Field::set_entity`](crate::game::spaces::field::Field::set_entity) to keep that in sync.
	pub fn set_position(&mut self, new_x: i16, new_y: i16) {
		self.x = new_x;
		self.y = new_y;
	}

	/// Position and id together: `(x, y, id)`.
	#[inline]
	pub fn get(&self) -> (i16, i16, EntityId) {
		(self.x, self.y, self.id)
	}

	/// The entity's grid position.
	#[inline]
	pub fn get_position(&self) -> (i16, i16) {
		(self.x, self.y)
	}

}

impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({},{})", self.id, self.x, self.y)
	}
}


/// The numbers an entity kind exposes to game logic — health and power. A thin
/// seam over the stats a [`being::Being`] carries, so combat-ish code can read
/// and write them without knowing the concrete type behind the entity.
pub trait EntityData {
	fn get_health(self) -> i32;
	fn set_health(&mut self, health:i32);

	fn get_power(self) -> i32;
	fn set_power(&mut self, attack_power:i32);
}
