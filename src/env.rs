use std::prelude::v1::*;

#[cfg(feature = "tstd")]
use env_logger_sgx as env_logger;

use crate::{LevelFilter, ReqId};
use chrono::{DateTime, NaiveDateTime, Utc};
use env_logger::Env;
use std::io::Write;
use std::time::SystemTime;

pub fn init_test() {
    let _ = env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] [{}:{}] [{}] - {}",
                format_date(&get_date()),
                ReqId::get_or_new(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .is_test(true)
        .filter(Some("rustls::client"), LevelFilter::Info)
        .filter(Some("rustls::server"), LevelFilter::Info)
        .try_init();
}

fn format_date(date: &DateTime<Utc>) -> String {
    use chrono::format::Numeric::*;
    use chrono::format::Pad::Zero;
    use chrono::format::{Fixed, Item};

    const PREFIX: &[Item<'static>] = &[
        Item::Numeric(Year, Zero),
        Item::Literal("-"),
        Item::Numeric(Month, Zero),
        Item::Literal("-"),
        Item::Numeric(Day, Zero),
        Item::Literal(" "),
        Item::Numeric(Hour, Zero),
        Item::Literal(":"),
        Item::Numeric(Minute, Zero),
        Item::Literal(":"),
        Item::Numeric(Second, Zero),
    ];

    let ssitem = Item::Fixed(Fixed::Nanosecond3);
    date.format_with_items(PREFIX.iter().chain([ssitem].iter()))
        .to_string()
}

fn get_date() -> DateTime<Utc> {
    let du = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let nano = du.subsec_nanos() / 1_000_000 * 1_000_000;
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(du.as_secs() as i64, nano).unwrap(),
        Utc,
    )
}

pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] [{}:{}] [{}] - {}",
                format_date(&get_date()),
                match std::thread::current().name() {
                    Some(name) => name.into(),
                    None => std::format!("{}", ReqId::get_or_new()),
                },
                record.module_path().unwrap_or("unknown"),
                // file_name,
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        // .filter(None, LevelFilter::Debug)
        .filter(Some("rustls::client"), LevelFilter::Info)
        .filter(Some("rustls::server"), LevelFilter::Info)
        .init();
}
