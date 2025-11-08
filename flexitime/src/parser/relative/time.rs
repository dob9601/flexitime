use chrono::{Months, NaiveDateTime};

use crate::error::{FlexitimeError, FlexitimeResult};

#[derive(Debug, PartialEq, Default)]
pub struct RelativeTime {
    pub seconds: Option<u32>,
    pub minutes: Option<u32>,
    pub hours: Option<u32>,
    pub days: Option<u32>,
    pub weeks: Option<u32>,
    pub months: Option<u32>,
    pub years: Option<u32>,
    pub negative: bool,
}

impl RelativeTime {
    pub fn new() -> Self {
        RelativeTime::default()
    }

    pub fn ago(mut self) -> Self {
        self.negative = true;
        self
    }

    pub fn hence(mut self) -> Self {
        self.negative = false;
        self
    }

    pub fn seconds(mut self, seconds: u32) -> Self {
        self.seconds = Some(seconds);
        self
    }

    pub fn minutes(mut self, minutes: u32) -> Self {
        self.minutes = Some(minutes);
        self
    }

    pub fn hours(mut self, hours: u32) -> Self {
        self.hours = Some(hours);
        self
    }

    pub fn days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }

    pub fn weeks(mut self, weeks: u32) -> Self {
        self.weeks = Some(weeks);
        self
    }

    pub fn months(mut self, months: u32) -> Self {
        self.months = Some(months);
        self
    }

    pub fn years(mut self, years: u32) -> Self {
        self.years = Some(years);
        self
    }

    pub fn to_chrono(&self, mut base_time: NaiveDateTime) -> NaiveDateTime {
        let sign = if self.negative { -1 } else { 1 };

        if let Some(seconds) = self.seconds {
            base_time += chrono::Duration::seconds((seconds as i64) * sign);
        }
        if let Some(minutes) = self.minutes {
            base_time += chrono::Duration::minutes((minutes as i64) * sign);
        }
        if let Some(hours) = self.hours {
            base_time += chrono::Duration::hours((hours as i64) * sign);
        }
        if let Some(days) = self.days {
            base_time += chrono::Duration::days((days as i64) * sign);
        }
        if let Some(weeks) = self.weeks {
            base_time += chrono::Duration::weeks((weeks as i64) * sign);
        }
        if let Some(months) = self.months {
            let months = Months::new(months);
            base_time = if self.negative {
                base_time.checked_sub_months(months)
            } else {
                base_time.checked_add_months(months)
            }
            .unwrap();
        }
        if let Some(years) = self.years {
            let months = Months::new(years * 12);
            base_time = if self.negative {
                base_time.checked_sub_months(months)
            } else {
                base_time.checked_add_months(months)
            }
            .unwrap();
        }

        base_time
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

    #[test]
    fn test_to_datetime() {
        let base_time = Utc::now().naive_utc();

        let time = RelativeTime::new()
            .years(1)
            .months(2)
            .weeks(3)
            .days(4)
            .hours(5)
            .minutes(6)
            .seconds(7)
            .to_chrono(base_time.clone());

        let mut new_time = base_time
            + Duration::seconds(7)
            + Duration::minutes(6)
            + Duration::hours(5)
            + Duration::days(4)
            + Duration::weeks(3);
        new_time = new_time.checked_add_months(Months::new(2)).unwrap();
        new_time = new_time.checked_add_months(Months::new(1 * 12)).unwrap();

        assert_eq!(time, new_time)
    }

    #[test]
    fn test_to_datetime_negative() {
        let base_time = Utc::now().naive_utc();

        let time = RelativeTime::new()
            .years(1)
            .months(2)
            .weeks(3)
            .days(4)
            .hours(5)
            .minutes(6)
            .seconds(7)
            .ago()
            .to_chrono(base_time.clone());

        let mut new_time = base_time
            - Duration::seconds(7)
            - Duration::minutes(6)
            - Duration::hours(5)
            - Duration::days(4)
            - Duration::weeks(3);
        new_time = new_time.checked_sub_months(Months::new(2)).unwrap();
        new_time = new_time.checked_sub_months(Months::new(1 * 12)).unwrap();

        assert_eq!(time, new_time)
    }
}
