#[derive(Copy, Clone)]
pub enum CHARSETS {
	Charset0 = 0,
	Charset1 = 1,
	Charset2 = 2,
	Charset3 = 3
}

pub fn get_charset(i:CHARSETS) -> String{
	let _charset_0:Vec<char> = vec!['.',',','-','=','+','*','#','▓'];

	let _charset_1:Vec<char> = vec!['░','▒','▓'];

	let _charset_2:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
								'k','l','m','n','o','p','q','r','s','t',
								'u','v','w','x','y','z'];
	
	let _charset_3:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
							 'k','l','m','n','o','p','q','r','s','t',
							 'u','v','w','x','y','z','1','2','3','4',
							 '5','6','7','8','9','0','#','*','.',',',
							 '+','=','-',' ',' ',' ',' ',' ',' ',' ',

							 ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
							 ' ',' ',' ',' '];
	
	
	let sets:Vec<_> = vec![_charset_0, _charset_1, _charset_2, _charset_3];
	
	let r = match i {
		CHARSETS::Charset0 => sets.get(0).unwrap(),
		CHARSETS::Charset1 => sets.get(1).unwrap(),
		CHARSETS::Charset2 => sets.get(2).unwrap(),
		CHARSETS::Charset3 => sets.get(3).unwrap(),
	};
	
	r.iter().collect::<String>()

}