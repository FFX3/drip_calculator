macro_rules! string_to_chrono {
    ($string:expr) => {{
        match dateparser::parse(&$string) {
            Ok(date) => {
                let without_hms = Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0,0,0).unwrap();
                Some(without_hms)
            },
            _ => None
        }
    }};
}
