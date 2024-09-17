use chrono::NaiveDate;

/// `NaiveDate` に変換可能な型を表すトレイト
pub trait DateLike {
    fn date(&self) -> Option<NaiveDate>;
}

impl DateLike for String {
    fn date(&self) -> Option<NaiveDate> {
        match NaiveDate::parse_from_str(self, "%Y-%m-%d") {
            Ok(parsed_date) => Some(parsed_date),
            Err(_) => match NaiveDate::parse_from_str(self, "%Y/%m/%d") {
                Ok(parsed_date) => Some(parsed_date),
                Err(_) => None,
            },
        }
    }
}

impl DateLike for &str {
    fn date(&self) -> Option<NaiveDate> {
        self.to_string().date()
    }
}

impl DateLike for NaiveDate {
    fn date(&self) -> Option<NaiveDate> {
        Some(*self)
    }
}

impl DateLike for Option<NaiveDate> {
    fn date(&self) -> Option<NaiveDate> {
        self.as_ref().map(|date| *date)
    }
}

impl DateLike for time::Date {
    fn date(&self) -> Option<NaiveDate> {
        let year = self.year();
        let month = self.month() as u32;
        let day = self.day() as u32;

        NaiveDate::from_ymd_opt(year, month, day)
    }
}

impl DateLike for Result<time::Date, time::error::ComponentRange> {
    fn date(&self) -> Option<NaiveDate> {
        match self {
            Ok(date) => date.date(),
            Err(_) => None,
        }
    }
}
