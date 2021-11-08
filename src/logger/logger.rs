use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

/// Representation of a log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// The item to log. Stored in little endian.
    pub item: Vec<u8>,
    /// The duration at which the log was ran since the program start.
    pub time: Duration,
}

/// A log file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogFile {
    pub logs: Vec<Log>,
}

impl LogFile {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }
}

pub struct Logger {
    log_current_duration: Duration,
    log_last_tick: Instant,
    logs: LogFile,
    replay_current_duration: Duration,
    replay_idx: usize,
    replay_last_tick: Instant,
    replay_logs: LogFile,
}

impl Logger {
    /// Loads a replay
    pub fn load_replay<'a>(&mut self, replay_path: &'a str) {
        self.replay_current_duration = Duration::from_micros(0);
        self.replay_idx = 0;
        self.replay_last_tick = Instant::now();

        let mut file = File::open(replay_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        self.replay_logs = serde_json::from_str(&contents).unwrap();
    }

    /// Creates a new logger
    pub fn new() -> Self {
        Self {
            log_current_duration: Duration::from_micros(0),
            log_last_tick: Instant::now(),
            logs: LogFile::new(),
            replay_current_duration: Duration::from_micros(0),
            replay_idx: 0,
            replay_last_tick: Instant::now(),
            replay_logs: LogFile::new(),
        }
    }

    /// Logs the given item
    pub fn log<'a>(&mut self, item: &'a [u8]) {
        let now = Instant::now();
        self.log_current_duration += now - self.log_last_tick;
        self.log_last_tick = now;

        let log = Log {
            item: item.iter().map(|l| l.to_le()).collect(),
            time: self.log_current_duration,
        };

        self.logs.logs.push(log);
    }

    /// Saves the log file
    pub fn save_logs<'a>(&mut self, log_path: &'a str) {
        let mut file = match File::create(log_path) {
            Ok(f) => f,
            Err(e) => panic!("Error saving Logger logs: {:?}", e),
        };

        let serialized = serde_json::to_string(&self.logs).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    /// Reads a replay item with the bytes read
    pub fn read_replay<'a>(&mut self, buffer: &mut [u8]) -> Option<usize> {
        // increment time since last read
        let now = Instant::now();
        self.replay_current_duration += now - self.replay_last_tick;
        self.replay_last_tick = now;

        // Check if there are more replays to use
        if self.replay_logs.logs.len() > self.replay_idx {
            let log = &self.replay_logs.logs[self.replay_idx];

            // If the current duration is greater or equal to the log time, return it and prep the next replay
            if self.replay_current_duration >= log.time {
                self.replay_idx += 1;

                if buffer.len() < log.item.len() {
                    panic!(
                        "BUFFER OVERFLOW for item {:?}! Buffer was {buffer_len} bytes, item was {item_len} bytes.",
                        log = log,
                        buffer_len = buffer.len(),
                        item_len = log.item.len()
                    )
                }

                let mut count = 0;
                for byte in log.item.iter() {
                    buffer[count] = u8::from_le(*byte);

                    count += 1;
                }

                return Some(count);
            }
        }

        None
    }
}
