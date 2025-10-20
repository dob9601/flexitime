use super::relative::time::RelativeTime;

#[derive(Debug, PartialEq)]
pub enum ParsedTime {
    Relative(RelativeTime),
    Absolute(chrono::DateTime<chrono::Utc>),
}
