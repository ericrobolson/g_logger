#![cfg_attr(not(feature = "log"), no_std)]

#[cfg(feature = "log")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "log")]
mod logger;

/// A class to log or replay items.
pub struct Logger;

impl Logger {
    /// Logs the given item
    #[allow(unused_variables)]
    pub fn log<'a>(item: &'a [u8]) {
        #[cfg(feature = "log")]
        {
            logger::log(item)
        }
    }

    /// Creates a new logger.
    /// `log_path` is the path to save the logs to.
    /// `replay_path` is the path to attempt to read replays from.
    #[allow(unused_variables)]
    pub fn load_replay<'a>(replay_path: &'a str) {
        #[cfg(feature = "log")]
        {
            logger::load_replay(replay_path);
        }
    }

    /// Attempts to replay an item. Returns `Some` if it was read with the bytes read, `None` if it was not.
    #[allow(unused_variables)]
    pub fn read_replay<'a>(buffer: &mut [u8]) -> Option<usize> {
        #[cfg(feature = "log")]
        {
            return logger::read_replay(buffer);
        }

        None
    }

    /// Saves the logs to the provided file
    #[allow(unused_variables)]
    pub fn save_logs<'a>(file: &'a str) {
        #[cfg(feature = "log")]
        {
            logger::save_logs(file);
        }
    }
}
