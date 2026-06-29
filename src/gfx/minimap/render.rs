use ratatui::style::{Color, Modifier};

use crate::{
	game::{entity::Entity, spaces::field::Field},
	gfx::screen::{Cell, Screen},
};

/// Glyph painted for a cell the watcher cannot currently see. The unseen is
/// shown, not omitted — the map must say where we cannot look, never pretend the
/// dark is empty floor. It is laid down as a translucent wash, so when this map
/// is composed over another panel the fog dims what is behind rather than
/// blanking it.
const FOG: char = '░';

/// Handles rendering entities from a Field to a screen buffer
#[derive(Clone)]
pub struct Render {
	render: Screen,
}

impl ToString for Render {
	fn to_string(&self) -> String {
		self.render.to_string()
	}
}

impl Render {
	/// The rendered panel as styled text, ready to hand to a ratatui widget.
	/// Fog reads as a dim wash; entity glyphs paint in the default colour.
	pub fn to_text(&self) -> ratatui::text::Text<'static> {
		self.render.to_text()
	}
}

impl Render {
	/// Creates a new render with specified dimensions
	pub fn init(width: u16, height: u16) -> Self {
		Self {
			render: Screen::new(width, height),
		}
	}
		/// Renders all entities from the field to the screen buffer
	pub fn rasterize(&mut self, field: &Field) {
		self.rasterize_window(field, 0, 0, |_, _| true);
	}

	/// Renders the field scrolled so the panel's top-left cell is world
	/// `(origin_x, origin_y)`. The view follows the origin, so the player can be
	/// kept in frame as they move — the world slides past instead of them walking
	/// off the edge. No fog gate (yet).
	pub fn rasterize_at(&mut self, field: &Field, origin_x: i16, origin_y: i16) {
		self.rasterize_window(field, origin_x, origin_y, |_, _| true);
	}

	/// Renders the field through a visibility gate, at world origin (0,0). A cell
	/// the watcher can see shows its highest-priority entity (or blank floor); a
	/// cell it cannot see shows [`FOG`]. `is_visible(x, y)` is supplied by the
	/// caller — e.g. backed by [`crate::game::vision::can_see`] — so the watcher's
	/// map has a notion of the unseen instead of revealing the whole field at once.
	pub fn rasterize_visible<F: Fn(i16, i16) -> bool>(&mut self, field: &Field, is_visible: F) {
		self.rasterize_window(field, 0, 0, is_visible);
	}

	/// Shared core: paint the panel-sized window whose top-left is world
	/// `(origin_x, origin_y)`, gated by `is_visible(world_x, world_y)`. The centered,
	/// omniscient, and scrolled rasterizers differ only in the origin they pass.
	fn rasterize_window<F: Fn(i16, i16) -> bool>(&mut self, field: &Field, origin_x: i16, origin_y: i16, is_visible: F) {
		// Wipe the panel back to bare paper before repainting.
		self.render.clear();

		// Pre-allocate with estimated capacity
		let estimated_capacity = (self.render.x as usize * self.render.y as usize / 10).max(field.entities.len());
		let mut position_map: std::collections::HashMap<(i16, i16), &Entity> =
			std::collections::HashMap::with_capacity(estimated_capacity);

		// First pass: collect highest priority entity at each position
		for entity in field.entities.values() {
			let pos = (entity.x, entity.y);

			match position_map.get(&pos) {
				Some(existing) => {
					if entity.priority > existing.priority {
						position_map.insert(pos, entity);
					}
				},
				None => {
					position_map.insert(pos, entity);
				}
			}
		}

		// Paint each cell by coordinate, gated by what the watcher can see. An
		// unseen cell gets a fog wash; a seen cell shows its entity (opaque ink)
		// or is left as bare paper.
		let fog = Cell::wash(FOG, Color::DarkGray, Modifier::DIM);
		for y in 0..self.render.y {
			for x in 0..self.render.x {
				let (wx, wy) = (x as i16 + origin_x, y as i16 + origin_y);
				if !is_visible(wx, wy) {
					self.render.set(x, y, fog);
				} else if let Some(entity) = position_map.get(&(wx, wy)) {
					self.render.put(x, y, entity.self_);
				}
			}
		}
	}
}


//Good reminder for me to say: tests are documentation for now
#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::Priority;

	fn one_entity_field() -> Field {
		let mut field = Field::new();
		field.add_entity(Entity::new(1, 1, 'F', 1, Priority::MED));
		field
	}

	#[test]
	fn all_visible_reveals_the_entity() {
		let mut r = Render::init(5, 3);
		r.rasterize_visible(&one_entity_field(), |_, _| true);
		let out = r.to_string();
		assert!(out.contains('F'), "a fully-lit map should reveal the entity");
		assert!(!out.contains(FOG), "nothing is fogged when everything is visible");
	}

	#[test]
	fn all_hidden_fogs_the_entity_away() {
		let mut r = Render::init(5, 3);
		r.rasterize_visible(&one_entity_field(), |_, _| false);
		let out = r.to_string();
		assert!(!out.contains('F'), "an unseen entity must not appear on the watcher's map");
		assert!(out.contains(FOG), "the unseen reads as fog, not as empty floor");
	}

	#[test]
	fn default_rasterize_stays_omniscient() {
		let mut r = Render::init(5, 3);
		r.rasterize(&one_entity_field());
		let out = r.to_string();
		assert!(out.contains('F'));
		assert!(!out.contains(FOG));
	}

	#[test]
	fn rasterize_at_scrolls_the_window_to_follow_the_origin() {
		let mut field = Field::new();
		field.add_entity(Entity::new(10, 10, 'P', 1, Priority::MED));
		let mut r = Render::init(5, 5);

		// At origin (0,0) the 5x5 window is world [0,5): the far entity is off-screen.
		r.rasterize_at(&field, 0, 0);
		assert!(!r.to_string().contains('P'), "an entity outside the window is not drawn");

		// Scroll the window onto it — top-left world (8,8) covers (10,10).
		r.rasterize_at(&field, 8, 8);
		assert!(r.to_string().contains('P'), "scrolling the origin brings the world into view");
	}
}
