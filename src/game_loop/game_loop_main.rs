use crate::utils::{logger::Logger, time::Time};
use crate::game::spaces::world::World;

pub struct GameLoopMain{
	tick:i64,
	state:GameStates,
	pub start: Time,
	pub logger:Logger,
	pub world:World,
	output:String
}

#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Exit = 2
}

impl GameLoopMain {
	pub fn new(start_time:Time) -> GameLoopMain {
		GameLoopMain { 
			tick: 0, 
			state: GameStates::Init,
			start:start_time.clone(),
			logger: Logger::new(start_time.clone()),
			world: World::new(),
			output:String::new()
		}
	}

	pub fn state_loop(&mut self) -> () {
		match self.state {
			GameStates::Exit => self.exit(),
			GameStates::Run  => self.run(),
			GameStates::Init => self.init(),
		}
	}

	pub fn init(&mut self) {
		self.logger.log("Initializing...");
		
		//neeed multithreading here, am lazy
		self.output = self.world.clone().init(self.logger.clone());
		self.tick += 1;

		self.logger.log("Initializing done");

		self.state = GameStates::Run;
		self.state_loop();
	}

	pub fn run(&mut self){
		
		let tick_max = 20;//10f64.powf(127.0);
		
		//element que
		//gen world closure
		//gen one sub group
		//transform the sub groups

		loop{
			self.logger.log("Running");
			
			if self.tick > tick_max as i64{
				break
			}
			self.tick += 1;

		}
		self.state = GameStates::Exit;
		self.state_loop();
	}

	pub fn exit(&mut self){

		self.logger.log(&self.output);

		self.logger.log("Exiting");
		std::process::exit(0x0);
	}

	pub fn draw_menu(mut self){
		
		self.state = GameStates::Exit;
		self.state_loop();
	}
}