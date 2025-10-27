use chrono::NaiveDateTime;

use super::{absolute::AbsoluteTime, relative::time::RelativeTime};

#[derive(Debug, PartialEq)]
pub enum ParsedTime {
    Relative(RelativeTime),
    Absolute(AbsoluteTime),
}

impl ParsedTime {
    pub fn to_chrono(&self) -> NaiveDateTime {
        match self {
            ParsedTime::Relative(time) => todo!(),
            ParsedTime::Absolute(time) => time.to_chrono(),
        }
    }
}
