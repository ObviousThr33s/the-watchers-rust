
use crate::utils::file_object::{self, FileObject};
use crate::logic::event::Event;

pub enum GameStates {
	Exit,
	Init,
	Running
}

pub struct GameState{
	game_state:GameStates,
	running:bool,
	save_file:FileObject
}

impl GameState {

	pub fn run(mut self) {
		
		while self.running {
			//possibly put a hashtable like thing here for events
			self.running = Event::get_input();
		}



		self.game_state = GameStates::Exit;
		self.update();
	}

	pub fn init(mut self) {
		self.running = true;
		self.game_state = GameStates::Running;

		self.update();
	}
	
	pub fn exit(mut current_state:GameState){
		current_state.running = false;
	}

	pub fn update(mut self/*, saveFile:FileObject*/) {
		match self.game_state{
			GameStates::Init    => {Self::init(self)},
			GameStates::Running => {Self::run(self)}
			GameStates::Exit    => {Self::exit(self)}
		}

		//saveFile;
	}

	pub  fn new() -> GameState {
		let gs:GameState = GameState {
			game_state:GameStates::Init,
			running:false,
			save_file:FileObject::new()
		};
		
		gs
	}


	
}