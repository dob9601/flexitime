use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, Utc};

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

impl AbsoluteTime {
    pub fn to_chrono(&self) -> NaiveDateTime {
        let mut datetime = Utc::now().naive_utc();

        datetime = match &self.date {
            FlexiDate::Date(date) => date.and_time(datetime.time()),
            FlexiDate::DayOffset(DayOffset::Fixed(days)) => {
                if *days < 0 {
                    datetime.checked_sub_days(Days::new(-days as u64)).unwrap()
                } else {
                    datetime.checked_add_days(Days::new(*days as u64)).unwrap()
                }
            }
            FlexiDate::DayOffset(DayOffset::NextDayOccurrence(weekday)) => {
                let current_day = datetime.weekday();
                let offset = 7 - current_day.days_since(weekday.clone());
                datetime.checked_add_days(Days::new(offset.into())).unwrap()
            }
        };

        datetime
    }
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
