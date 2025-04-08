pub struct FileObject{
	_id:i64
}

impl FileObject{
	pub fn new() -> FileObject {
		
		let f = FileObject{
			_id:0
		};

		f
	}
}