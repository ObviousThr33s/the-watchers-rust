
pub mod utils{
    //use std::any::Any;

    use std::process::Command;

	pub struct Screen {
		size_x: i32,
		size_y: i32
	}

	impl Screen {
		
		pub fn new(s_x: i32, s_y: i32) -> Screen {
			Screen{
				size_x: s_x,
				size_y: s_y,
			}
		}

		pub fn to_string(s:Screen){
			println!("[SX:{}, SY:{}]",s.size_x,s.size_y);
		}
	}

	struct Stream{ 
		active: bool,
		x: i32,
		y: i32,
		c: char
	}

	pub fn render(ticks: u32){
		print!("Conan Obrian once said about Jorge Klinmtoon that, the j");
		
		let mut cursor = Stream {
			active : true,
			x: 0,
			y: 0,
			c: '.'
		};
		
		println!();
		print!("\x1B[2J\x1B[1;1H");

		
		(1..10).for_each(|_m| {
			(1..10).for_each(|_n| {
				if cursor.active == true {
					
					cursor.c = char::from_u32(ticks).unwrap();
				
					print!("{}", cursor.c);
				}
				cursor.y = _n;
			});
			println!();
			cursor.x = _m; 
			// /println!("{}{}", cursor.x, cursor.y);
		});

	}

	pub fn put(s: String){
		print!("{}",s);
	}



}