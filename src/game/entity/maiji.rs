//! One being kind: the wandering fairy (Ooloonoo and her twin Oolooroo). Built
//! inert, then filled from a `.being` file — the data there is the source of
//! truth, and this is only the shell their behavior hangs on.

use crate::game::entity::{ Actor, Entity, EntityData};
use crate::game::entity::being::Being;

/// A being-in-the-world: an in-field [`Entity`] (glyph + position) bound to an
/// [`Actor`] (stats, art, the line they speak). Built inert and brought to life
/// by [`apply_being`](Self::apply_being).
pub struct Fairy {
	pub entity:Entity,
	pub actor:Actor
}

impl Clone for Fairy {
	fn clone(&self) -> Self {
		Self {
			entity: self.entity.clone(),
			actor: self.actor.clone()
		}
	}
}

impl Fairy {
	/// A fairy is built *before* its `.being` is loaded, so `new` fills only
	/// inert placeholders. The real glyph, stats, and art arrive from the
	/// `.being` file via [`apply_being`] — that file is the source of truth, not
	/// this code. The placeholder glyph is a deliberately-wrong `?` so a fairy
	/// that was never applied (e.g. the file failed to load) shows up on the map
	/// as "unloaded" instead of masquerading as the real thing. The placeholder
	/// health stays positive so a missing `.being` degrades to a visible
	/// placeholder rather than being reaped as dead.
	pub fn new(x:i16, y:i16, name:String, id:String) -> Self{
		const PLACEHOLDER_GLYPH: char = '?';
		const PLACEHOLDER_STAT: i32 = 1;
		Self {
			entity: Entity::new(
				x,
				y,
				PLACEHOLDER_GLYPH,
				id,
				crate::game::entity::Priority::MED,
			),
			actor:Actor {
				name,
				health: PLACEHOLDER_STAT,
				attack_power: PLACEHOLDER_STAT,
				art:String::new(),
				prompt:String::new(),
			}
		}
	}
}

impl Fairy {
	/// Overlay a `.being` definition onto this fairy: the file is the source of
	/// truth for name, stats, glyph, and art. This is the seam where the
	/// data-driven model starts driving the running game.
	pub fn apply_being(&mut self, being: &Being) {
		self.actor.name = being.name.clone();
		self.actor.health = being.health;
		self.actor.attack_power = being.power;
		self.actor.art = being.art.clone();
		self.actor.prompt = being.line.clone();
		self.entity.self_ = being.glyph;
	}

	/// A placeholder wander: every third tick, jump to a random nearby cell and
	/// lose a point of health. A stand-in from before the data-driven behavior
	/// model — kept only until gaze-gated behaviors (move when unwatched) replace it.
	pub fn warp(&mut self, tick:usize) {
		if tick%3 == 0 {
			self.entity.set_position(rand::random_range(0..10), rand::random_range(0..10));
			self.actor.health -= 1;
		}
	}
}

impl EntityData for Fairy {
	fn get_health(self) -> i32{
		self.actor.health
	}

	fn set_health(&mut self, health:i32) {
		self.actor.health = health;
	}
	
	fn get_power(self) -> i32 {
		self.actor.attack_power
	}
	
	fn set_power(&mut self, attack_power:i32) {
		self.actor.attack_power = attack_power
	}
}