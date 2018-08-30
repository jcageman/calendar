use chrono;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = chrono::Duration;
pub type Date = chrono::Date<chrono::Utc>;
pub type Time = chrono::NaiveTime;
pub const MIN_DATE: Date = chrono::MIN_DATE;
pub const MAX_DATE: Date = chrono::MAX_DATE;
