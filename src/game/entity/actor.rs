//! The actor: an entity's mutable game-state — the combat numbers, plus the art
//! and prompt it reveals when seen — reached through the [`ActorData`] accessors.
//! This is the *running copy* of what a `.being` file defines; the file stays the
//! source of truth (see `apply_being` on the kinds that wear an actor).

/// An actor's live state: name, the combat numbers, and the art/prompt surfaced
/// when it comes into view. Filled from a `.being` definition; edited in place
/// as the game runs.
pub struct Actor{
	pub(crate) name: String,
	pub(crate) health: i32,
	pub(crate) attack_power: i32,
	pub art:String,
	pub prompt:String
}

impl Clone for Actor {
	fn clone(&self) -> Self {
		Self {	
			name: self.name.clone(),
			health: self.health, 
			attack_power: self.attack_power, 
			art: self.art.clone(), 
			prompt: self.prompt.clone() 
		}
	}
}

impl ActorData for Actor {
	fn name(&self) -> &str {
		&self.name
	}

	fn name_mut(&mut self) -> &mut String {
		&mut self.name
	}

	fn health(&self) -> i32 {
		self.health
	}

	fn health_mut(&mut self) -> &mut i32 {
		&mut self.health
	}

	fn attack_power(&self) -> i32 {
		self.attack_power
	}

	fn attack_power_mut(&mut self) -> &mut i32 {
		&mut self.attack_power
	}

	fn art(&self) -> &str {
		&self.art
	}

	fn art_mut(&mut self) -> &mut String {
		&mut self.art
	}

	fn new(name: String, health: i32, attack_power: i32) -> Self {
		Actor { name, health, attack_power, art:String::new(), prompt:String::new() }
	}

	fn set_name(&mut self, name:String) {
		self.name = name
	}
}

/// Accessors over an actor's state — name, health, attack power, art — plus a
/// constructor. The `*_mut` methods hand back a mutable borrow so a caller can
/// edit one field in place. A trait rather than bare fields, so any entity kind
/// can present the same handle to game logic.
pub trait ActorData {
	fn name(&self) -> &str;
	fn name_mut(&mut self) -> &mut String;
	fn health(&self) -> i32;
	fn health_mut(&mut self) -> &mut i32;
	fn attack_power(&self) -> i32;
	fn attack_power_mut(&mut self) -> &mut i32;
	fn art(&self) -> &str;
	fn art_mut(&mut self) -> &mut String;

	fn new(name: String, health: i32, attack_power: i32) -> Self;

	fn set_name(&mut self, name: String);

	/// A quick `(name, health)` snapshot for read-outs such as the Stats panel.
	/// Has a default body, so implementors get it for free.
	fn get_stats(&self) -> (String, i32) {
		(self.name().to_string(), self.health())
	}
}