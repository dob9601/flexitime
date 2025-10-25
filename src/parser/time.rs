use super::{absolute::AbsoluteTime, relative::time::RelativeTime};

#[derive(Debug, PartialEq)]
pub enum ParsedTime {
    Relative(RelativeTime),
    Absolute(AbsoluteTime),
}
