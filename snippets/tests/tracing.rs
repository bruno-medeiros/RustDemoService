mod common;

use std::thread;
use std::time::Duration;
use time::UtcOffset;
use tracing::{info, warn, Level};
use tracing_subscriber::fmt::time::OffsetTime;

#[test]
#[ignore]
fn tracing_subscriber_default() {
    tracing_subscriber::fmt().try_init().ok();

    info!("Hello, world!");
}

#[test]
fn tracing_subscriber_commons() {
    common::init_logging();

    info!("Hello, world!");
    thread::sleep(Duration::from_millis(200));
    warn!("Warning!!!");
}


#[test]
#[ignore]
fn tracing_subscriber_short_time() {
    use time::macros::format_description;

    let offset = UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second].[subsecond digits:5]"));

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_timer(timer)
        .try_init()
        .ok();

    info!("Hello, world!");
    thread::sleep(Duration::from_millis(200));
    warn!("Warning!!!");
}

