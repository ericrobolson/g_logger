A library for logging events and replaying them. All serialized bytes are in little endian.

To use enable the 'log' feature. Otherwise this defaults to a `no_std` crate.

To use enable the feature `log`.

Call `Logger::log(bytes)` to log an item.

Call `Logger::save_logs(file_path)` to dump the log file.

Call `Logger::load_replay(file_path)` to read a log file.

Call `Logger::read_replay(mut buffer)` to attempt to read an event.

Roadmap:
- [ ] Return `Result<_, Error>` instead of panics