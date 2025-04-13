
pub struct GameFile {
	_id:i64,
	path:String
}

pub trait FileObject {
	fn new() -> Self;
}

impl  Clone for  GameFile {
	fn clone(&self) -> Self {
		Self { _id: self._id.clone(), path: self.path.clone() }
	}
}

impl FileObject for GameFile {
	fn new() -> Self {
		GameFile { _id: 0, path:String::new() }
	}
}
