use chrono_tz::Tz;

use super::relative::time::RelativeTime;

#[derive(Debug, PartialEq)]
pub enum ParsedTime {
    Relative(RelativeTime),
    Absolute(chrono::DateTime<Tz>),
}
