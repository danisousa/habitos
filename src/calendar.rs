use chrono::{DateTime, FixedOffset, NaiveDate, TimeDelta};

const DAY_STARTS_AT_HOUR: i64 = 4;

pub fn game_date(at: DateTime<FixedOffset>) -> NaiveDate {
    (at.naive_local() - TimeDelta::hours(DAY_STARTS_AT_HOUR)).date()
}
