use chrono::{NaiveDateTime, Utc};

use super::{absolute::AbsoluteTime, relative::time::RelativeTime};

#[derive(Debug, PartialEq)]
pub enum ParsedTime {
    Relative(RelativeTime),
    Absolute(AbsoluteTime),
}

impl ParsedTime {
    pub fn to_chrono(&self) -> NaiveDateTime {
        match self {
            ParsedTime::Relative(time) => time.to_chrono(Utc::now().naive_utc()).unwrap(),
            ParsedTime::Absolute(time) => time.to_chrono(),
        }
    }
}
