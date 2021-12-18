use chrono::prelude::*;

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
    let start = format!("{}-{}-{}", local.year(), local.month() - 1, local.day());
    start
}
