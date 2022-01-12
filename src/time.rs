use chrono::prelude::*;

/// Returns the current date in YYYY-MM-DD format as a String
/// # Example
/// ```
/// use voyager_client::time;
///
/// let today = time::today();
///
/// ```
pub fn today() -> String {
    let local: DateTime<Local> = Local::now();
    let today = format!("{}-{}-{}", local.year(), local.month(), local.day());
    today
}

pub fn one_day() -> String {
    let local: DateTime<Local> = Local::now();
    let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 1);
    start
}

pub fn one_week() -> String {
    let local: DateTime<Local> = Local::now();
    let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 7);
    start
}

pub fn two_weeks() -> String {
    let local: DateTime<Local> = Local::now();
    let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 14);
    start
}

pub fn one_month() -> String {
    let local: DateTime<Local> = Local::now();
    if local.month() == 1 {
        let last_month = 12;
        let start = format!("{}-{}-{}", local.year() - 1, last_month, local.day());
        start
    } else {
        let start = format!("{}-{}-{}", local.year(), local.month() - 1, local.day());
        start
    }
}