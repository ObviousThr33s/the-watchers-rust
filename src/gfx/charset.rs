#[derive(Copy, Clone)]
pub enum CHARSETS {
	Charset0 = 0,
	Charset1 = 1,
	Charset2 = 2,
}

//wrapper code for getting the charsets, should be from file 

pub fn get_charset(i:CHARSETS) -> String{
	let _charset_0:Vec<char> = vec!['.',',','-','=','+','*','#','▓'];

	let _charset_1:Vec<char> = vec!['░','▒','▓'];

	let _charset_2:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
									'k','l','m','n','o','p','q','r','s','t',
									'u','v','w','x','y','z'];

	
	let sets:Vec<_> = vec![_charset_0, _charset_1, _charset_2];
	
	let r = match i {
		CHARSETS::Charset0 => sets.get(0).unwrap(),
		CHARSETS::Charset1 => sets.get(1).unwrap(),
		CHARSETS::Charset2 => sets.get(2).unwrap(),
	};
	
	r.iter().collect::<String>()

}

pub fn get_charset_vec(i:CHARSETS) -> Vec<char>{
	let _charset_0:Vec<char> = vec!['.',',','-','=','+','*','#','▓'];

	let _charset_1:Vec<char> = vec!['░','▒','▓'];

	let _charset_2:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
									'k','l','m','n','o','p','q','r','s','t',
									'u','v','w','x','y','z'];

	
	let sets:Vec<_> = vec![_charset_0, _charset_1, _charset_2];
	
	let r = match i {
		CHARSETS::Charset0 => sets.get(0),
		CHARSETS::Charset1 => sets.get(1),
		CHARSETS::Charset2 => sets.get(2),
	};

	let char_set:Vec<char> = r.unwrap().clone();
	char_set

}