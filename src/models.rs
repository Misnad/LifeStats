use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct Man {
    pub nickname: String,
    pub birth: NaiveDate,
}
