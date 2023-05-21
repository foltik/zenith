use std::env;
use std::time::Instant;
use log::{Level, LevelFilter};
use yansi::Paint;

pub use ::log::{error, warn, info, debug, trace};

struct Logger {
    start: Instant,
}

impl log::Log for Logger {
    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            Level::Error => Paint::red("ERROR"),
            Level::Warn => Paint::yellow("WARN "),
            Level::Info => Paint::green("INFO "),
            Level::Debug => Paint::blue("DEBUG"),
            Level::Trace => Paint::cyan("TRACE"),
        };
        let time = self.start.elapsed().as_secs_f64();
        let source = record.module_path_static().unwrap_or("unknown");

        let message = record.args();

        println!("[{level} +{time:03.3} {source}] {message}");
    }
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn flush(&self) {}
}

pub(crate) fn init() {
    log::set_max_level(match env::var("RUST_LOG") {
        Ok(level) => level.parse().expect("unknown log level"),
        Err(_) => LevelFilter::Info,
    });

    log::set_logger(Box::leak(Box::new(Logger {
        start: Instant::now(),
    }))).expect("failed to set logger");
}
