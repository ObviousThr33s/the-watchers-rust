//! The game world and its clock. [`Game`] owns the [`Field`] every entity stands
//! in and the [`Haps`] event queue that paces a tick. `init` populates the world
//! (the player, a starter alcove, a sown forest); `update` is where per-tick
//! behavior will land as the rebuild continues. Pure engine — everything it
//! places (beings, flora) is data loaded from `.being` files, never hardcoded.

/*pub mod trading;
pub mod crafting; //includes resource gathering

pub mod dialogue;
pub mod skills;
pub mod combat;

pub mod items;
pub mod levels;

pub mod entities;
pub mod world;*/

use spaces::field::Field;
use spaces::terrain::{self, Sowing};
use spaces::heightmap::NoiseGround;
use haps::{Event, Haps};

use crate::{utils::logger};

pub mod entity;
pub mod spaces;
pub mod poly;
pub mod vision;
pub mod recollection;
pub mod haps;
pub mod rover;
pub mod fairy;
pub mod item;

//pub mod group;

/// The world: the [`Field`] of everything placed in it, plus the per-tick event
/// queue ([`Haps`], named `time` for the tempo it keeps).
pub struct Game {
	pub field:Field,
	/// The ground's altitude under the whole world — the heightmap the voxel view
	/// reads as relief. A seeded noise field for now; generation deepens here later
	/// (octaves, water and plateau bounds) without the renderer or field noticing.
	pub ground:NoiseGround,
	/// The fairy that flits around the forest — set into and lifted out of the
	/// field as it comes and goes (see [`flit_fairy`](Self::flit_fairy)).
	pub fairy:fairy::Fairy,
	/// Items the player is carrying — picked up off the ground, set back down on
	/// command (see [`step_player`](Self::step_player) and [`drop_at`](Self::drop_at)).
	pub inventory:Vec<item::Item>,
	/// Items still lying in the field, by which the world tells a pick-up-able item
	/// apart from a wall or a tree.
	ground_items:Vec<item::Item>,
	/// The per-tick event bus (wards 1–2): a fixed-capacity ring of pure-data
	/// [`Event`]s. Systems push facts onto it during the read phase;
	/// [`dispatch`](Self::dispatch) drains and applies them. Named `time` for the
	/// tempo a drained queue keeps.
	pub time:Haps,
}

impl Game {
	/// An empty world — a bare field and an empty event queue. Call [`init`](Self::init)
	/// (or the first [`update`](Self::update)) to populate it.
	pub fn new() -> Self {
		let mut field = Field::new();
		// The fairy takes a stable id up front (minted before any flora), so the
		// world can set it down and lift it off cleanly as it flits.
		let fairy = fairy::Fairy::new(field.mint(), (8, 8));
		Game {
			field,
			// One seed grows one world; the same seed always grows the same ground.
			ground: NoiseGround::new(1),
			fairy,
			inventory: Vec::new(),
			ground_items: Vec::new(),
			time: Haps::new(),
		}
	}

	/// Populate the world for the first time — place the player, raise the starter
	/// alcove, and sow the forest from the flora `.being` files. Meant to run once,
	/// at tick 0 (see [`update`](Self::update)).
	pub fn init(&mut self, logger: &mut logger::Logger) {
		// The player is the one hardcoded spawn (see entity::player) — the lens
		// the world is seen through, not data like the beings. Place them in the
		// field so the renderer always has, at minimum, the player to draw. The
		// rest of the population (beings, behaviors) is the rebuild still to come.
		self.field.add_entity(entity::player::Player::new().player);

		// Structural geometry (not narrative): a small alcove around the player
		// so the first-person view has walls to cast against. The player faces
		// "up" (-y) from (2,2), so the front wall sits at y=0 dead ahead with a
		// wall down each side. Scaffolding for the view — this will likely move to
		// per-field level data later, not stay hardcoded here.
		let walls = [
			(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), // front wall, dead ahead
			(0, 1), (0, 2),                         // left wall
			(4, 1), (4, 2),                         // right wall
		];
		for (x, y) in walls {
			let id = self.field.mint();
			self.field.add_entity(entity::Entity::new(
				x, y, '#', id, entity::Priority::LOW,
			));
		}

		logger.log(&format!("Player placed; {} walls built", walls.len()));

		// Sow the field with flora — trees, brush, and brambles loaded from their
		// `.being` files. The story (glyph, art, the line each surfaces when seen)
		// lives in those files; this only scatters them into groves. They're solid,
		// so the forest is at once scenery on the Map and walls the view casts
		// against. The clearing keeps the spawn alcove open. (The hardcoded alcove
		// above can retire once the forest alone gives the view enough to cast on.)
		let flora = terrain::load_flora();
		let planted = terrain::sow(&mut self.field, &flora, Sowing {
			x0: 0,
			y0: 0,
			width: 48,
			height: 30,
			clear_around: (2, 2),
			clearing: 3,
			threshold: 0.35,
			scale: 0.18,
			seed: 1,
		});
		logger.log(&format!("Sowed {planted} flora from {} kinds", flora.len()));

		// One thing to find close at hand: a lantern set just to the player's left,
		// inside the open alcove, so stepping left picks it up.
		self.place_item(1, 2, '!', "a small lantern");

		// And a scattered hoard to wander after, strewn across the sown region on a
		// fixed seed so the same world always holds the same finds.
		let strewn = self.strew_items(0xB0B, 16, 0, 0, 48, 30);
		logger.log(&format!("Strewed {strewn} items across the world"));
	}

	/// Advance the world by one tick. At tick 0 it bootstraps via [`init`](Self::init);
	/// the per-tick event/behavior pipeline (drain [`Haps`] in arrival order, run
	/// gaze-gated behavior) is the rebuild still ahead, so for now this only seeds
	/// the world. `recollection` and `logger` are threaded in ahead of that wiring.
	pub fn update(&mut self, tick: usize, logger: &mut logger::Logger) {
		// Build this tick's event queue, then apply the events in arrival order.
		
		if tick == 0 {
			self.init(logger);
		}

		
		// Art/prompt still flow out through parameters, so generation stays a
		// direct call rather than an event for now. Folding it in is the next
		// pass, once art/prompt live in game state instead of being threaded out.
	}

	/// Advance the fairy one beat: lift it off wherever it stood, then set it back
	/// down on its new haunt — but only on open ground, so it slips between the
	/// trees and never clobbers one. When it has slipped out of sight it simply
	/// stays lifted. This is what makes it appear and disappear around the forest.
	pub fn flit_fairy(&mut self) {
		self.field.remove_entity(self.fairy.id);
		if let Some((x, y)) = self.fairy.flit() {
			if self.field.get_entity_by_position(x, y).is_none() {
				self.field.add_entity(entity::Entity::new(
					x, y, fairy::FAIRY_GLYPH, self.fairy.id, entity::Priority::MED,
				));
			}
		}
	}

	/// Lay an item in the field at `(x, y)` and remember it as pick-up-able,
	/// returning the id it was minted with.
	pub fn place_item(&mut self, x: i16, y: i16, glyph: char, name: &str) -> entity::EntityId {
		let id = self.field.mint();
		self.field.add_entity(entity::Entity::new(x, y, glyph, id, entity::Priority::LOW));
		self.ground_items.push(item::Item { id, glyph, name: name.to_owned() });
		id
	}

	/// If a pick-up-able item lies at `(x, y)`, take it out of the field and into the
	/// inventory. Returns whether anything was picked up. A wall or a tree there is
	/// left alone — only items the world laid down are carried.
	pub fn pick_up_at(&mut self, x: i16, y: i16) -> bool {
		let Some(id) = self.field.get_entity_by_position(x, y).map(|e| e.id) else {
			return false;
		};
		let Some(at) = self.ground_items.iter().position(|it| it.id == id) else {
			return false;
		};
		let item = self.ground_items.remove(at);
		self.field.remove_entity(id);
		self.inventory.push(item);
		true
	}

	/// Set the most-recently-carried item down at `(x, y)`, if that cell is open.
	/// Returns whether anything was dropped.
	pub fn drop_at(&mut self, x: i16, y: i16) -> bool {
		if self.field.get_entity_by_position(x, y).is_some() {
			return false; // can't set it down on something solid
		}
		let Some(item) = self.inventory.pop() else {
			return false; // empty pockets
		};
		self.field.add_entity(entity::Entity::new(x, y, item.glyph, item.id, entity::Priority::LOW));
		self.ground_items.push(item);
		true
	}

	/// Step the player by `(dx, dy)`, picking up any item on the destination cell as
	/// you enter it — you walk onto an item to take it. A wall still blocks the step.
	pub fn step_player(&mut self, dx: i16, dy: i16) {
		let Some((px, py)) = self.field.get_entity_by_id(entity::PLAYER).map(|e| e.get_position())
		else {
			return;
		};
		self.pick_up_at(px + dx, py + dy); // frees the cell if it held an item
		self.field.move_entity(entity::PLAYER, dx, dy);
	}

	/// Drop a carried item onto the cell the player faces (`facing` in the
	/// ray-caster convention). Returns whether anything was dropped.
	pub fn drop_ahead(&mut self, facing: f32) -> bool {
		let Some((px, py)) = self.field.get_entity_by_id(entity::PLAYER).map(|e| e.get_position())
		else {
			return false;
		};
		let dx = facing.cos().round() as i16;
		let dy = facing.sin().round() as i16;
		self.drop_at(px + dx, py + dy)
	}

	/// Strew up to `count` items across the `width`×`height` rectangle at
	/// `(x0, y0)`, choosing kind and place from `seed` so the same seed always
	/// scatters the same hoard. Items only land on open ground — never over a wall,
	/// a tree, or each other — so a crowded region may hold fewer than asked.
	/// Returns how many were actually laid down.
	pub fn strew_items(&mut self, seed: u64, count: usize, x0: i16, y0: i16, width: i16, height: i16) -> usize {
		use rand::{rngs::StdRng, Rng, SeedableRng};

		let mut rng = StdRng::seed_from_u64(seed);
		let (w, h) = (width.max(1), height.max(1));
		let mut placed = 0;
		// Bounded tries, so a packed region can't spin forever hunting open ground.
		for _ in 0..count.saturating_mul(40) {
			if placed >= count {
				break;
			}
			let x = x0 + rng.random_range(0..w);
			let y = y0 + rng.random_range(0..h);
			if self.field.get_entity_by_position(x, y).is_some() {
				continue;
			}
			let (glyph, name) = item::KINDS[rng.random_range(0..item::KINDS.len())];
			self.place_item(x, y, glyph, name);
			placed += 1;
		}
		placed
	}

	/// Phases 2 and 3 of a tick (see the wards in `CLAUDE.md`): drain the event
	/// queue in arrival order and apply each event to the field. [`apply`](Self::apply)
	/// is handed the field *alone*, never the queue, so a handler structurally
	/// cannot raise a new event mid-drain — ward 3's "no event raised during the
	/// mutation phase", made impossible by construction rather than by discipline.
	pub fn dispatch(&mut self) {
		while let Some(event) = self.time.pop() {
			Self::apply(&mut self.field, event);
		}
	}

	/// Apply one drained event to the world. Pure mutation over the field and the
	/// event's own data — nothing borrowed, nothing global. Today only
	/// [`SpawnSekaikan`](Event::SpawnSekaikan) carries everything it needs to act;
	/// the rest wait on systems still being rebuilt and are honest no-ops until then.
	fn apply(field: &mut Field, event: Event) {
		match event {
			Event::SpawnSekaikan { x, y, id } => {
				// Place the being's inert shell, addressed by the id the event
				// carried. Its `.being` (glyph, name, stats, art) is overlaid later
				// by `apply_being`; component storage for the Actor half doesn't
				// exist yet, so only the in-field Entity lands here.
				let shell = entity::sekaikan::Sekaikan::new(x, y, String::new(), id);
				field.add_entity(shell.entity);
			}
			// AdvanceWatchers waits on the gaze-gated behavior pass; ReapDead waits
			// on health living in the field (it sits on the Actor today, not the
			// Entity). Honest no-ops until those systems return.
			Event::AdvanceWatchers | Event::ReapDead => {}
		}
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::gfx::minimap::render::Render;
	use crate::utils::{logger::Logger, time::Time};

	/// The renderer drew nothing because the field was empty after the refactor.
	/// init must put the player in the world — the floor the rest of the rebuild
	/// (beings, behaviors, gaze) stands on.
	#[test]
	fn init_places_the_player_in_the_field() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		assert!(
			game.field.get_entity_by_id(entity::PLAYER).is_some(),
			"init must place the player in the field"
		);
	}

	#[test]
	fn the_fairy_comes_and_goes_in_the_field() {
		let mut game = Game::new();
		let (mut present, mut absent) = (0, 0);
		for _ in 0..12 {
			game.flit_fairy();
			if game.field.get_entity_by_id(game.fairy.id).is_some() {
				present += 1;
			} else {
				absent += 1;
			}
		}
		assert!(present > 0, "the fairy alights in the field on some beats");
		assert!(absent > 0, "and is gone from it on others");
	}

	#[test]
	fn an_item_is_picked_up_by_walking_onto_it_and_can_be_dropped() {
		let mut game = Game::new();
		game.field.add_entity(entity::Entity::new(2, 2, '^', entity::PLAYER, entity::Priority::MED));
		game.place_item(3, 2, '!', "a small lantern");

		assert!(game.inventory.is_empty(), "pockets start empty");

		// Step right, onto the item.
		game.step_player(1, 0);
		assert_eq!(game.inventory.len(), 1, "walking onto an item picks it up");
		assert_eq!(
			game.field.get_entity_by_id(entity::PLAYER).map(|e| e.get_position()),
			Some((3, 2)),
			"and the player ends on the now-cleared cell",
		);

		// Drop it onto the open cell to the right.
		assert!(game.drop_at(4, 2), "a carried item drops onto open ground");
		assert!(game.inventory.is_empty(), "and leaves the pocket");
		assert_eq!(
			game.field.get_entity_by_position(4, 2).map(|e| e.self_),
			Some('!'),
			"landing back in the field where it was set down",
		);
	}

	#[test]
	fn an_item_cannot_be_dropped_onto_something_solid() {
		let mut game = Game::new();
		game.field.add_entity(entity::Entity::new(2, 2, '^', entity::PLAYER, entity::Priority::MED));
		let lantern = game.place_item(3, 2, '!', "a small lantern");
		game.step_player(1, 0); // pick it up

		// A wall sits at (4,2); the drop must refuse and keep the item in hand.
		game.field.add_entity(entity::Entity::new(4, 2, '#', 999, entity::Priority::LOW));
		assert!(!game.drop_at(4, 2), "a blocked cell refuses the drop");
		assert_eq!(game.inventory.first().map(|i| i.id), Some(lantern), "the item stays carried");
	}

	#[test]
	fn the_item_generator_is_seeded_and_lays_them_on_open_ground() {
		let cells = |g: &Game| -> Vec<(i16, i16, char)> {
			let mut v: Vec<_> = g
				.ground_items
				.iter()
				.filter_map(|it| g.field.get_entity_by_id(it.id).map(|e| (e.x, e.y, e.self_)))
				.collect();
			v.sort();
			v
		};

		let mut a = Game::new();
		let na = a.strew_items(0xB0B, 12, 0, 0, 20, 20);
		let mut b = Game::new();
		let nb = b.strew_items(0xB0B, 12, 0, 0, 20, 20);

		assert!(na > 0, "the generator actually lays items down");
		assert_eq!(na, nb, "the same seed strews the same count");
		assert_eq!(cells(&a), cells(&b), "and in the same places, with the same kinds");

		// No two share a cell — each landed on open ground.
		let mut spots: Vec<(i16, i16)> = cells(&a).into_iter().map(|(x, y, _)| (x, y)).collect();
		let before = spots.len();
		spots.dedup();
		assert_eq!(before, spots.len(), "no two strewn items share a cell");
	}

	/// The render pipeline itself is sound (see gfx::minimap::render tests); the
	/// bug was an empty field, so it painted nothing. With the player placed,
	/// rasterizing the field must show the player's glyph — content, not a blank
	/// panel. This is the regression guard for "the renderer isn't working."
	#[test]
	fn renderer_draws_what_init_populates() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		let player = game
			.field
			.get_entity_by_id(entity::PLAYER)
			.expect("init must place the player")
			.clone();

		let mut lamp = Render::init(20, 20);
		lamp.rasterize(&game.field);

		assert!(
			lamp.to_string().contains(player.self_),
			"the renderer should draw the player glyph, not an empty panel"
		);
	}

	/// The bus's whole reason to exist: a fact raised in one phase becomes a change
	/// to the world in a later one. Queue a spawn and nothing moves; *drain* it, and
	/// the being's shell stands in the field under the very id the event named —
	/// proof the queue defers, applying facts on dispatch, never on push.
	#[test]
	fn dispatch_spawns_the_being_an_event_named() {
		let mut game = Game::new();
		let id = game.field.mint();
		game.time
			.push(Event::SpawnSekaikan { x: 7, y: 3, id })
			.expect("a fresh ring has room for one event");

		// Still only queued — the world is unchanged until dispatch runs.
		assert!(
			game.field.get_entity_by_id(id).is_none(),
			"a pushed spawn must wait for the mutation phase"
		);

		game.dispatch();

		let spawned = game
			.field
			.get_entity_by_id(id)
			.expect("dispatch must place the spawned being in the field");
		assert_eq!(spawned.get_position(), (7, 3), "it lands where the event said");
	}

	/// The first-person view needs walls to cast against; an empty field renders
	/// as blank space. init must build the alcove so the live viewport shows a
	/// surface dead ahead of the (up-facing) player.
	#[test]
	fn init_builds_a_room_so_the_first_person_view_has_walls() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		assert!(
			game.field.get_entity_by_position(2, 0).is_some(),
			"expected a wall dead ahead of the up-facing player at (2,0)"
		);
	}
}
