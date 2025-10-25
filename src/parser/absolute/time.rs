use chrono::NaiveDate;

use super::{day_offset::DayOffset, wallclock_time::WallClockTime};

#[derive(Debug, PartialEq, Clone)]
pub enum FlexiDate {
    Date(NaiveDate),
    DayOffset(DayOffset),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AbsoluteTime {
    time: WallClockTime,
    date: FlexiDate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AbsoluteTimeBuilder {
    time: Option<WallClockTime>,
    date: Option<FlexiDate>,
}

impl AbsoluteTimeBuilder {
    pub fn new() -> Self {
        Self {
            time: None,
            date: None,
        }
    }

    pub fn time(mut self, time: WallClockTime) -> Self {
        self.time = Some(time);
        self
    }

    pub fn date(mut self, date: FlexiDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn build(self) -> Result<AbsoluteTime, String> {
        match (self.time, self.date) {
            (Some(time), Some(date)) => Ok(AbsoluteTime { time, date }),
            _ => Err("Missing required fields".to_string()),
        }
    }
}
