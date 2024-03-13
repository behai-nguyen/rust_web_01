/* Date Created: 06/03/2024. */

//! The application logger.

use std::str::FromStr;

use time::macros::format_description;
use tracing::Level;
use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use tracing_subscriber::{
    fmt::{time::OffsetTime, Layer},
    layer::SubscriberExt,
};

// TRACE, DEBUG, INFO, WARN, ERROR.
//
// Level implements the PartialOrd and Ord traits, allowing two Levels to be compared 
// to determine which is considered more or less verbose. Levels which are more verbose 
// are considered “greater than” levels which are less verbose, with Level::ERROR considered
// the lowest, and Level::TRACE considered the highest.
// 
//     assert!(Level::TRACE > Level::DEBUG);
//     assert!(Level::ERROR < Level::WARN);
//     assert!(Level::INFO <= Level::DEBUG);
//
// https://docs.rs/tracing/latest/tracing/struct.Level.html
//
// fn with_min_level(self, level: Level) -> WithMinLevel<Self>
//    will only write output for events at or above the provided verbosity Level.
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/writer/trait.MakeWriterExt.html#method.with_min_level
//
// fn with_max_level(self, level: Level) -> WithMaxLevel<Self>
//    will only write output for events at or below the provided verbosity Level. 
//    Level::TRACE is considered to be _more verbosethanLevel::INFO
//    Events whose level is more verbose than level will be ignored
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/writer/trait.MakeWriterExt.html#method.with_max_level

/// Setting up the application logger.
/// 
/// # Arguments
/// 
/// * `utc_offset` - local date time offsetto UTC. The Australian Eastern Standard Time (AEST)
/// is between 10 and 11 hours ahead of UTC. The value of ``utc_offset`` is 10 or 11.
/// 
/// Calculating this offset with ``UtcOffset::current_local_offset().unwrap()`` raises 
/// [IndeterminateOffset](https://docs.rs/time/latest/time/error/struct.IndeterminateOffset.html) 
/// error. 
/// 
/// See also [Document #293 in local-offset feature description #297](https://github.com/time-rs/time/pull/297).
/// 
/// # Return
/// 
/// - [WorkerGuard](https://docs.rs/tracing-appender/latest/tracing_appender/non_blocking/struct.WorkerGuard.html).
///     This guard must be kept alive during the live of the application server for the log to work.
/// 
/// # Note
/// 
/// Without ``log_appender.with_max_level(level)`` the logging would not work correctly. That is, 
/// having only ``RUST_LOG=error`` in the ``.env`` file does seem not enough.
/// 
pub fn init_app_logger(utc_offset: time::UtcOffset) -> WorkerGuard {
    let log_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) // Daily log file.
        .filename_suffix("log") // log file names will be suffixed with `.log`
        .build("./log") // try to build an appender that stores log files in `/var/log`
        .expect("Initialising rolling file appender failed");

    let (non_blocking_appender, log_guard) = tracing_appender::non_blocking(log_appender);

    // Each log line starts with a local date and time token.
    // 
    // On Ubuntu 22.10, calling UtcOffset::current_local_offset().unwrap() after non_blocking()
    // causes IndeterminateOffset error!!
    // 
    // See also https://github.com/time-rs/time/pull/297.
    //
    let timer = OffsetTime::new(
        //UtcOffset::current_local_offset().unwrap(),
        utc_offset,
        format_description!("[year]-[month]-[day]-[hour]:[minute]:[second]"),
    );
    
    // Extracts tracing::Level from .env RUST_LOG, if there is any problem, 
    // defaults to Level::DEBUG.
    //
    let level: Level = match std::env::var_os("RUST_LOG") {
        None => Level::DEBUG,

        Some(text) => {
            match Level::from_str(text.to_str().unwrap()) {
                Ok(val) => val,
                Err(_) => Level::DEBUG
            }
        }
    };

    let subscriber = tracing_subscriber::registry()
        .with(
            Layer::new()
                .with_timer(timer)
                .with_ansi(false)
                .with_writer(non_blocking_appender.with_max_level(level)
                    .and(std::io::stdout.with_max_level(level)))
        );

    // tracing::subscriber::set_global_default(subscriber) can only be called once. 
    // Subsequent calls raise SetGlobalDefaultError, ignore these errors.
    //
    // There are integeration test methods which call this init_app_logger(...) repeatedly!!
    //
    match tracing::subscriber::set_global_default(subscriber) {
        Err(err) => tracing::error!("Logger set_global_default, ignored: {}", err),
        _ => (),
    }

    tracing::debug!("RUST_LOG level {}", level);

    log_guard
}
