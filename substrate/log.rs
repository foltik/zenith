// use ::log::{Level, LevelFilter};
// use std::env;
// use std::time::Instant;
// use yansi::Paint;

// pub use ::log::{debug, error, info, trace, warn};

// struct Logger {
//     start: Instant,
// }

// impl log::Log for Logger {
//     fn log(&self, record: &log::Record) {
//         let level = match record.level() {
//             Level::Error => Paint::red("ERROR"),
//             Level::Warn => Paint::yellow("WARN "),
//             Level::Info => Paint::green("INFO "),
//             Level::Debug => Paint::blue("DEBUG"),
//             Level::Trace => Paint::cyan("TRACE"),
//         };
//         let time = self.start.elapsed().as_secs_f64();
//         let source = record.module_path_static().unwrap_or("unknown");

//         let message = record.args();

//         println!("[{level} +{time:.3} {source}] {message}");
//     }
//     fn enabled(&self, _: &log::Metadata) -> bool {
//         true
//     }
//     fn flush(&self) {}
// }

use std::time::Instant;

use tracing::metadata::LevelFilter;

pub use tracing::{debug, error, info, trace, warn};
pub use tracing::{event, span, Level};
use tracing_subscriber::fmt::time::FormatTime;

#[derive(thiserror::Error)]
struct InitError {
    #[error("failed to set logger")]
    SetLoggerError,
    #[error("invalid log filter")]
}

pub fn init(module: &str, level: i8, filter: Option<String>) -> Result<(), InitError> {
    let level = match level {
        ..=-3 => LevelFilter::OFF,
        -2 => LevelFilter::ERROR,
        -1 => LevelFilter::WARN,
        0 => LevelFilter::INFO,
        1 => LevelFilter::DEBUG,
        2.. => LevelFilter::TRACE,
    };

    let directive = match filter {
        Some(filter) => filter,
        None => todo!(),
    };

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive()
        .from_env_lossy();

    let formatter = tracing_subscriber::fmt()
        .with_timer(Uptime {
            start: Instant::now(),
        })
        .finish();

    // log::set_max_level(match env::var("RUST_LOG") {
    //     Ok(level) => level.parse().expect("unknown log level"),
    //     Err(_) => LevelFilter::Info,
    // });

    // log::set_logger(Box::leak(Box::new(Logger {
    //     start: Instant::now(),
    // })))
    // .expect("failed to set logger");

    tracing::subscriber::set_global_default(formatter)
        .expect("failed to install tracing subscriber")
}

struct Uptime {
    start: Instant,
}

impl FormatTime for Uptime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{:3}.{:03}",
            self.start.elapsed().as_secs(),
            self.start.elapsed().subsec_millis()
        )
    }
}
