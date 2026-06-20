use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::time::Time;

#[derive(Clone)]
pub struct Logger {
	pub tick: usize,
	pub start_time: Time,

	log_stream: Vec<String>,
	vers: String,
}

impl Logger {
	pub fn new(start_time: Time, version: String) -> Self {
		Logger {
			log_stream: Vec::new(),
			start_time,
			tick: 0,
			vers: version,
		}
	}

	/// Append one timestamped line: `[YYYY-MM-DD HH:MM:SS.mmm]<tick> message`.
	/// The stamp is taken at the moment the line is written, so the log reads as
	/// a true timeline rather than every entry sharing the run's start time.
	pub fn log(&mut self, message: &str) {
		let stamp = Time::timestamp();
		self.log_stream.push(format!("[{stamp}]<{}> {message}\n", self.tick));
		self.tick += 1;
	}

	/// The log so far, newest line first. Borrows rather than consuming `self`,
	/// so callers (e.g. the live UI) no longer have to clone the whole logger to
	/// read it.
	pub fn get_log(&self) -> Vec<String> {
		let mut lines = self.log_stream.clone();
		lines.reverse();
		lines
	}

	pub fn get_version(&self) -> &str {
		&self.vers
	}

	/// Write the full log to `res/logs/log_<session>.txt` — one file per run,
	/// named for when the run began, so successive sessions don't overwrite one
	/// another. Led by a header naming the start time and version. On the way
	/// out, so an I/O failure is reported to stderr rather than panicking over
	/// the exit.
	pub fn save_log(&self) {
		let dir = "./res/logs";
		let mut file_path = PathBuf::from(dir);
		file_path.push(format!("log_{}.txt", self.start_time.file_stamp()));

		if let Err(e) = std::fs::create_dir_all(dir) {
			eprintln!("Failed to create directory: {e}");
			return;
		}

		let mut f: File = match File::create(file_path) {
			Ok(file) => file,
			Err(e) => {
				eprintln!("Failed to create file: {e}");
				return;
			}
		};

		let header = format!(
			"# session started {} — v{}\n",
			self.start_time.started_at(),
			self.vers,
		);
		let body = self.log_stream.join("");

		if let Err(e) = f.write_all(header.as_bytes()).and_then(|_| f.write_all(body.as_bytes())) {
			eprintln!("Failed to write log: {e}");
		}
	}
}
