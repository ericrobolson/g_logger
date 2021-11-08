mod logger;
use logger::Logger;

use std::{borrow::BorrowMut, sync::Mutex};

lazy_static! {
    /// Create a single logger instance that can be logged to.
    pub(crate) static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

/// Loads a replay file
pub fn load_replay<'a>(replay_path: &'a str) {
    LOGGER.lock().unwrap().borrow_mut().load_replay(replay_path);
}

/// Logs the given item
pub fn log<'a>(item: &'a [u8]) {
    LOGGER.lock().unwrap().borrow_mut().log(item);
}

/// Attempts to read a replay item and returns the bytes read
pub fn read_replay<'a>(buffer: &mut [u8]) -> Option<usize> {
    LOGGER.lock().unwrap().borrow_mut().read_replay(buffer)
}

/// Logs the given item
pub fn save_logs<'a>(file_path: &'a str) {
    LOGGER.lock().unwrap().borrow_mut().save_logs(file_path);
}
