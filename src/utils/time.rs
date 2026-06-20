use std::time::SystemTime;
use chrono::prelude::DateTime;
use chrono::Utc;

/// The clock for one run: when it began (`started_at`), plus the wall-clock
/// stamp each log line carries (`timestamp`). Formatting is to millisecond
/// precision so two entries in the same second stay distinguishable.
#[derive(Clone, Copy)]
pub struct Time {
	start_time: SystemTime,
}

impl Time {
	pub fn new() -> Self {
		Time { start_time: SystemTime::now() }
	}

	/// Wall-clock time of *now*: `YYYY-MM-DD HH:MM:SS.mmm`. Stamped onto each
	/// log line as it is written.
	pub fn timestamp() -> String {
		Self::format(SystemTime::now(), "%Y-%m-%d %H:%M:%S%.3f")
	}

	/// When this run began, in the same format — used for the log file header.
	pub fn started_at(&self) -> String {
		Self::format(self.start_time, "%Y-%m-%d %H:%M:%S%.3f")
	}

	/// A filesystem-safe stamp of when this run began: `YYYY-MM-DD_HH-MM-SS`
	/// (no colons — those are illegal in Windows filenames). Names each
	/// session's log file so runs don't overwrite one another.
	pub fn file_stamp(&self) -> String {
		Self::format(self.start_time, "%Y-%m-%d_%H-%M-%S")
	}

	fn format(at: SystemTime, pattern: &str) -> String {
		let dt: DateTime<Utc> = at.into();
		dt.format(pattern).to_string()
	}
}
