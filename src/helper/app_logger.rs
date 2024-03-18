/* Date Created: 06/03/2024. */

//! The application logger.

use time::macros::format_description;
use time_tz::OffsetDateTimeExt;

use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};

use tracing_subscriber::{
    fmt::{time::OffsetTime, Layer, writer::MakeWriterExt}, 
    EnvFilter, layer::{SubscriberExt, Layer as _}
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
/// # Note on Local Time Offset Calculating
/// 
/// Please note this block of the code:
/// 
/// ```text
///     // Each log line starts with a local date and time token.
///     let system_tz = time_tz::system::get_timezone()
///         .expect("Failed to find system timezone");
///     let localtime = time::OffsetDateTime::now_utc().to_timezone(system_tz);
/// 
///     let timer = OffsetTime::new(
///         localtime.offset(),
///         format_description!("[year]-[month]-[day]-[hour]:[minute]:[second]"),
///     );
/// ```
/// 
/// Originally `localtime.offset()` was ``UtcOffset::current_local_offset().unwrap()``.
/// 
/// ``UtcOffset::current_local_offset().unwrap()`` raises 
/// [IndeterminateOffset](https://docs.rs/time/latest/time/error/struct.IndeterminateOffset.html) 
/// error. 
/// 
/// According to this GitHub issue on Dec 19, 2020, 
/// [Document #293 in local-offset feature description #297](https://github.com/time-rs/time/pull/297),
/// this problem has not been fixed.
/// 
/// See also the following issues:
/// 
/// * Nov 25, 2020 -- [Time 0.2.23 fails to determine local offset #296](https://github.com/time-rs/time/issues/296).
/// 
/// * Nov 2, 2021 -- [Better solution for getting local offset on unix #380](https://github.com/time-rs/time/issues/380).
/// 
/// * Dec 5, 2019 -- [tzdb support #193](https://github.com/time-rs/time/issues/193). 
///   [This reply](https://github.com/time-rs/time/issues/193#issuecomment-1037227056) on Feb 13, 2022
///   points to crate [time-tz](https://crates.io/crates/time-tz), which solves the above error.
/// 
/// # Return
/// 
/// - [WorkerGuard](https://docs.rs/tracing-appender/latest/tracing_appender/non_blocking/struct.WorkerGuard.html).
///     This guard must be kept alive during the live of the application server for the log to work.
/// 
/// # Note on ``RUST_LOG`` Environment Variable
/// 
/// Please see [Enabling logging](https://docs.rs/env_logger/latest/env_logger/#enabling-logging).
/// 
/// In the context of this application, some example of valid configurations are:
/// 
/// * ``RUST_LOG=off,learn_actix_web=debug``
/// * ``RUST_LOG=off,learn_actix_web=info`` -- nothing will get log.
/// * ``RUST_LOG=off,learn_actix_web=debug,actix_server=info``
/// 
pub fn init_app_logger() -> WorkerGuard {
    let log_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) // Daily log file.
        .filename_suffix("log") // log file names will be suffixed with `.log`
        .build("./log") // try to build an appender that stores log files in `/var/log`
        .expect("Initialising rolling file appender failed");

    let (non_blocking_appender, log_guard) = tracing_appender::non_blocking(log_appender);

    // Each log line starts with a local date and time token.
    let system_tz = time_tz::system::get_timezone()
        .expect("Failed to find system timezone");
    let localtime = time::OffsetDateTime::now_utc().to_timezone(system_tz);

    let timer = OffsetTime::new(
        localtime.offset(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    let subscriber = tracing_subscriber::registry()
        .with(
            Layer::new()
                .with_timer(timer)
                .with_ansi(false)
                .with_writer(non_blocking_appender
                    .and(std::io::stdout))
                .with_filter(filter_layer)
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

    log_guard
}
