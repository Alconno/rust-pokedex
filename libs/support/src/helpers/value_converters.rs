use chrono::NaiveDate;
use chrono::Datelike;

pub fn date_to_number(date: NaiveDate) -> i32 {
    let year = date.year() * 10_000;
    let month = date.month() as i32 * 100;
    let day = date.day() as i32;
    year + month + day
}