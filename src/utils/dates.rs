use chrono::{ NaiveDateTime};

pub fn convert_date(time: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_millis(time * 1000).unwrap()
}
