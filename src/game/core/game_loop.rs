use  crate::{logic::event::Event, utils::file::game_file::{SaveFile, SaveFileObject}};

use super::game_states::{GameStates};

pub struct GameLoop {
	pub game_state: GameStates,
	pub running: bool,
	pub save_file: SaveFile,
}
pub trait Loop {
	fn new() ->  Self;
	fn update(&mut self);
	fn init(&mut self);

	fn run(&mut self) {
		println!("Game is running...");
	}

	fn exit(&mut self) {
		println!("Game exited!");
	}
}

impl Loop for GameLoop {
	
	fn new() -> GameLoop {
		GameLoop {
			game_state:GameStates::Init,
			running: false,
			save_file: SaveFile::new(),
		}
	}

	fn run(&mut self) {
		
		while self.running {
			//possibly put a hashtable like thing here for events
			self.running = Event::get_input();
		}



		self.game_state = GameStates::Exit;
		self.update();
	}

	fn init(&mut self) {
		self.running = true;
		self.game_state = GameStates::Running;
		self.run();
	}
	
	fn exit(&mut self){
		self.running = false;
	}

	fn update(&mut self) {
		match self.game_state {
			GameStates::Init	 => { Self::init(self) },
			GameStates::Running => { Self::run(self) },
			GameStates::Exit	 => { Self::exit(self) },
		}

		// Use save_file to save the game state
		
		//self.save_file.clone().save();
		println!("foo");
	}

	
}