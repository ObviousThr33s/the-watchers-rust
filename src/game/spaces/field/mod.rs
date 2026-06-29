use std::collections::HashMap;

use crate::game::entity::{Entity, EntityId};

mod roster;
use roster::Roster;
//this must be able to translate to any UI system
/// Represents a 2D field where entities can be placed
#[derive(Clone)]
pub struct Field {
	/// The entity backbone: a fixed-capacity slab addressed by id (see [`Roster`]).
	pub entities: Roster,
	/// Spatial index for fast position-based lookups: (x, y) -> entity_id
	pub spatial_index: HashMap<(i16, i16), EntityId>,
	/// The next id [`mint`](Self::mint) will hand out. Starts at `1` so minted ids
	/// never collide with the player's fixed [`PLAYER`](crate::game::entity::PLAYER) (`0`).
	next_id: EntityId,
}

impl Field {
	/// Creates a new empty Field
	pub fn new() -> Self {
		Field {
			entities: Roster::new(),
			spatial_index: HashMap::new(),
			next_id: 1,
		}
	}

	/// Hands out a fresh, unique [`EntityId`]. The single source of new ids for
	/// everything the world places procedurally — walls, sown flora, and (later)
	/// spawned beings — so no two entities are ever minted the same handle.
	pub fn mint(&mut self) -> EntityId {
		let id = self.next_id;
		self.next_id += 1;
		id
	}

	/// Adds an entity to the field
	pub fn add_entity(&mut self, entity: Entity) {
		let pos = (entity.x, entity.y);
		let id = entity.id;

		// Remove any existing entity at this position
		if let Some(old_id) = self.spatial_index.insert(pos, id) {
			self.entities.remove(old_id);
		}

		self.entities.insert(entity);
	}

	/// Updates an existing entity in the field with position changes
	pub fn set_entity(&mut self, entity: Entity) {
		let new_pos = (entity.x, entity.y);
		let id = entity.id;

		// Remove old spatial index entry if entity exists and position changed
		if let Some(old_entity) = self.entities.get(id) {
			let old_pos = (old_entity.x, old_entity.y);
			if old_pos != new_pos {
				self.spatial_index.remove(&old_pos);
			}
		}

		// Update spatial index and entity
		self.spatial_index.insert(new_pos, id);
		self.entities.insert(entity);
	}

	/// Moves entity `id` by `(dx, dy)` if the destination cell is free, returning
	/// whether it actually moved. Every entity is solid (the ray caster treats
	/// them all as walls), so a step into any occupied cell is blocked. The
	/// spatial index is kept in sync via [`set_entity`].
	pub fn move_entity(&mut self, id: EntityId, dx: i16, dy: i16) -> bool {
		let mut moved = match self.get_entity_by_id(id) {
			Some(e) => e.clone(),
			None => return false,
		};
		let (nx, ny) = (moved.x + dx, moved.y + dy);
		if self.is_occupied(nx, ny, id) {
			return false; // blocked by a wall (or anything else solid)
		}
		moved.set_position(nx, ny);
		self.set_entity(moved);
		true
	}

	/// Gets an entity at a specific position (x, y), if it exists
	#[inline]
	pub fn get_entity_by_position(&self, x: i16, y: i16) -> Option<&Entity> {
		self.spatial_index.get(&(x, y))
			.and_then(|id| self.entities.get(*id))
	}

	/// Returns true if some entity other than `ignore_id` occupies (x, y).
	/// Every entity is treated as solid, matching how the ray caster renders
	/// them all as walls.
	#[inline]
	pub fn is_occupied(&self, x: i16, y: i16, ignore_id: EntityId) -> bool {
		self.get_entity_by_position(x, y)
			.is_some_and(|e| e.id != ignore_id)
	}

	/// Gets an entity by its ID, if it exists
	#[inline]
	pub fn get_entity_by_id(&self, id: EntityId) -> Option<&Entity> {
		self.entities.get(id)
	}

	/// Mutable lookup by id. Editing the position through this skips the spatial
	/// index — use [`move_entity`](Self::move_entity)/[`set_entity`](Self::set_entity) to keep it in sync.
	#[inline]
	pub fn get_entity_by_id_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
		self.entities.get_mut(id)
	}

	/// Remove entity `id` from both the map and the spatial index.
	pub fn remove_entity(&mut self, id: EntityId) {
		if let Some(entity) = self.entities.remove(id) {
			self.spatial_index.remove(&(entity.x, entity.y));
		}
	}

	/// Renders entities to an ASCII map of the given size, scrolled so that
	/// (center_x, center_y) sits in the middle. Centering on the player keeps
	/// them pinned to the middle of the minimap as they move around the world.
	pub fn to_ascii_map(&self, width: usize, height: usize, center_x: i16, center_y: i16) -> String {
		// World coordinate of the visible window's top-left corner.
		let origin_x = center_x - (width as i16) / 2;
		let origin_y = center_y - (height as i16) / 2;
		self.window_to_ascii(width, height, origin_x, origin_y)
	}

	/// Renders entities to an ASCII map snapped to a chunk grid: the world is tiled
	/// into `width`×`height` chunks, and the window shows whichever chunk contains
	/// `(at_x, at_y)`. The view holds still while you move inside a chunk and *snaps*
	/// a whole chunk over the moment you cross its edge — room-to-room, never a
	/// smooth scroll. (Chunk size equals the view for now; decoupling the two is the
	/// "viewport separate from chunk data" refinement.)
	pub fn to_chunk_map(&self, width: usize, height: usize, at_x: i16, at_y: i16) -> String {
		// Top-left of the chunk containing (at_x, at_y). `div_euclid` floors toward
		// negative infinity, so the snap stays correct on both sides of the origin;
		// `.max(1)` keeps a zero-width panel from dividing by zero.
		let chunk_w = (width as i16).max(1);
		let chunk_h = (height as i16).max(1);
		let origin_x = at_x.div_euclid(chunk_w) * chunk_w;
		let origin_y = at_y.div_euclid(chunk_h) * chunk_h;
		self.window_to_ascii(width, height, origin_x, origin_y)
	}

	/// Paint every entity falling inside the `width`×`height` window whose top-left
	/// world cell is `(origin_x, origin_y)`, as newline-joined ASCII rows. The shared
	/// core behind [`to_ascii_map`] (centered) and [`to_chunk_map`] (snapped); they
	/// differ only in how they choose the origin.
	fn window_to_ascii(&self, width: usize, height: usize, origin_x: i16, origin_y: i16) -> String {
		let mut output = vec![vec![' '; width]; height];

		for entity in self.entities.values() {
			let sx = entity.x - origin_x;
			let sy = entity.y - origin_y;

			// Only draw entities that fall inside the visible window.
			if sx >= 0 && (sx as usize) < width && sy >= 0 && (sy as usize) < height {
				output[sy as usize][sx as usize] = entity.self_;
			}
		}

		output
			.iter()
			.map(|row| row.iter().collect::<String>())
			.collect::<Vec<_>>()
			.join("\n")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::{Entity, Priority, PLAYER};

	#[test]
	fn minimap_centers_on_the_given_point() {
		let mut field = Field::new();
		field.add_entity(Entity::new(5, 5, '^', PLAYER, Priority::MED));
		field.add_entity(Entity::new(5, 4, '#', 1, Priority::LOW));

		// A 5x5 window centered on (5,5) has origin (3,3), so the player lands
		// dead center at (2,2) and the wall one row above it at (2,1).
		let map = field.to_ascii_map(5, 5, 5, 5);
		let rows: Vec<&str> = map.lines().collect();

		assert_eq!(rows[2].chars().nth(2), Some('^'), "player should be centered");
		assert_eq!(rows[1].chars().nth(2), Some('#'), "wall should sit one row above");
	}

	#[test]
	fn the_grid_snaps_a_whole_chunk_when_you_cross_its_edge() {
		let mut field = Field::new();
		// Landmarks one chunk apart: A at world (0,0) in chunk 0, B at (4,0) in chunk 1.
		field.add_entity(Entity::new(0, 0, 'A', 1, Priority::LOW));
		field.add_entity(Entity::new(4, 0, 'B', 2, Priority::LOW));

		// Standing inside chunk 0 (x=1): the window is [0,4); A shows, B is off-screen.
		let chunk0 = field.to_chunk_map(4, 1, 1, 0);
		assert_eq!(chunk0.chars().next(), Some('A'), "the current chunk's landmark shows");
		assert!(!chunk0.contains('B'), "the next chunk's landmark is off-screen");

		// Cross the edge to x=4 (chunk 1): the window snaps to [4,8); B shows, A is gone.
		let chunk1 = field.to_chunk_map(4, 1, 4, 0);
		assert_eq!(chunk1.chars().next(), Some('B'), "the view snapped a whole chunk over");
		assert!(!chunk1.contains('A'), "the chunk left behind fell away");
	}

	#[test]
	fn the_view_holds_still_while_you_move_inside_a_chunk() {
		let mut field = Field::new();
		field.add_entity(Entity::new(0, 0, 'A', 1, Priority::LOW));
		// x=1 and x=3 are both inside chunk [0,4): the view must not budge until the edge.
		let near = field.to_chunk_map(4, 1, 1, 0);
		let far = field.to_chunk_map(4, 1, 3, 0);
		assert_eq!(near, far, "moving within a chunk does not scroll the view");
	}

	#[test]
	fn move_entity_steps_into_open_space() {
		let mut field = Field::new();
		field.add_entity(Entity::new(2, 2, '^', PLAYER, Priority::MED));

		assert!(field.move_entity(PLAYER, 0, 1), "an open cell should be steppable");

		let p = field.get_entity_by_id(PLAYER).unwrap();
		assert_eq!(p.get_position(), (2, 3), "the player moved one cell down");
		assert!(field.get_entity_by_position(2, 3).is_some(), "spatial index follows the move");
		assert!(field.get_entity_by_position(2, 2).is_none(), "the old cell is vacated");
	}

	#[test]
	fn move_entity_is_blocked_by_a_wall() {
		let mut field = Field::new();
		field.add_entity(Entity::new(2, 2, '^', PLAYER, Priority::MED));
		field.add_entity(Entity::new(2, 1, '#', 1, Priority::LOW));

		assert!(!field.move_entity(PLAYER, 0, -1), "a wall must block the step");
		assert_eq!(
			field.get_entity_by_id(PLAYER).unwrap().get_position(),
			(2, 2),
			"a blocked player does not move"
		);
	}
}
