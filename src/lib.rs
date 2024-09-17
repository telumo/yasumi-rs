mod datelike;
mod public_holiday;

use chrono::{Datelike, NaiveDate};
use datelike::DateLike;
use public_holiday::{substitute_holiday, HOLIDAYS};

/// 「国民の休日」を含めない祝日を判定します.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 祝日の場合は祝日名を返します. それ以外の場合は `None` を返します.
/// ただし, 「国民の休日」は含まれません.
fn calc_holiday_without_national_holiday<T: DateLike>(date: T) -> Option<String> {
    let date = date.date()?;

    for holiday in HOLIDAYS.iter() {
        if holiday.is_holiday(&date) {
            return Some(holiday.name());
        }
    }

    // 振替休日
    if let Some(substitute) = substitute_holiday(&date) {
        return Some(substitute);
    }
    None
}

/// 祝日を判定します.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 祝日の場合は祝日名を返します. それ以外の場合は `None` を返します.
fn calc_holiday<T: DateLike>(date: T) -> Option<String> {
    let date = date.date()?;

    if let Some(holiday_name) = calc_holiday_without_national_holiday(date) {
        return Some(holiday_name);
    }

    // 日曜日の場合は国民の休日ではない
    if date.weekday().number_from_monday() == 7 {
        return None;
    }
    // 次の日と前の日が祝日の場合
    let next_day = date.succ_opt()?;
    let prev_day = date.pred_opt()?;
    let next_day_holiday = calc_holiday_without_national_holiday(next_day);
    let prev_day_holiday = calc_holiday_without_national_holiday(prev_day);

    if next_day_holiday.is_some() && prev_day_holiday.is_some() {
        return Some("国民の休日".to_string());
    }

    None
}

/// 指定した日付の祝日名を取得します.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 祝日の場合は祝日名を返します. それ以外の場合は `None` を返します.
///
/// # Examples
/// ```
/// use yasumi::is_holiday_name;
///
/// let holiday = is_holiday_name("2024-01-01");
/// assert_eq!(holiday, Some("元日".to_string()));
/// ```
pub fn is_holiday_name<T: DateLike>(date: T) -> Option<String> {
    let date = date.date()?;
    calc_holiday(date)
}

/// 指定した日付の祝日名を取得します.
/// is_holiday_name のエイリアスです.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 祝日の場合は祝日名を返します. それ以外の場合は `None` を返します.
///
/// # Examples
/// ```
/// use yasumi::holiday_name;
///
/// let holiday = holiday_name("2024-01-01");
/// assert_eq!(holiday, Some("元日".to_string()));
/// ```
pub fn holiday_name<T: DateLike>(date: T) -> Option<String> {
    is_holiday_name(date)
}

/// 指定した日付が祝日かどうかを判定します.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 祝日の場合は `true` を返します. それ以外の場合は `false` を返します.
///
/// # Examples
/// ```
/// use yasumi::is_holiday;
///
/// let holiday = is_holiday("2024-01-01");
/// assert_eq!(holiday, true);
/// ```
pub fn is_holiday<T: DateLike>(date: T) -> bool {
    is_holiday_name(date).is_some()
}

/// 指定した日付が土日祝かどうかを判定します.
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 土日祝の場合は `true` を返します. それ以外の場合は `false` を返します.
///
/// # Examples
/// ```
/// use yasumi::is_no_workday;
///
/// let no_workday = is_no_workday("2024-09-15"); // 日曜日
/// assert_eq!(no_workday, true);
/// ```
pub fn is_no_workday<T: DateLike>(date: T) -> bool {
    let date = date.date().unwrap();
    if date.weekday().number_from_monday() == 6 || date.weekday().number_from_monday() == 7 {
        return true;
    }
    is_holiday(date)
}

/// 指定した年月の祝日を取得します.
/// 祝日が存在しない場合は空のリストを返します.
///
/// # Arguments
///
/// * `year` - 年
/// * `month` - 月
///
/// # Returns
///
/// 祝日のリストを返します.
///
/// # Examples
/// ```
/// use yasumi::month_holidays;
///
/// let holidays = month_holidays(2024, 1);
/// assert_eq!(holidays.len(), 2);
/// ```
pub fn month_holidays(year: i32, month: u32) -> Vec<(NaiveDate, String)> {
    let mut holidays = vec![];

    for day in 1..=31 {
        let date = NaiveDate::from_ymd_opt(year, month, day);
        if let Some(date) = date {
            if let Some(name) = is_holiday_name(date) {
                holidays.push((date, name));
            }
        }
    }

    holidays
}

/// 指定した年の祝日を取得します.
/// 祝日が存在しない場合は空のリストを返します.
///
/// # Arguments
///
/// * `year` - 年
///
/// # Returns
///
/// 祝日のリストを返します.
///
/// # Examples
/// ```
/// use yasumi::year_holidays;
///
/// let holidays = year_holidays(2024);
/// assert_eq!(holidays.len(), 21);
/// ```
pub fn year_holidays(year: i32) -> Vec<(NaiveDate, String)> {
    let mut holidays = vec![];

    for month in 1..=12 {
        for day in 1..=31 {
            let date = NaiveDate::from_ymd_opt(year, month, day);
            if let Some(date) = date {
                if let Some(name) = is_holiday_name(date) {
                    holidays.push((date, name));
                }
            }
        }
    }

    holidays
}

/// 指定した期間の祝日を取得します.
/// 祝日が存在しない場合は空のリストを返します.
///
/// # Arguments
///
/// * `start_date` - 開始日(含む)
/// * `end_date` - 終了日(含む)
///
/// # Returns
///
/// 祝日のリストを返します.
///
/// # Examples
/// ```
/// use yasumi::holidays;
///
/// let holidays = holidays("2024-01-01", "2024-12-31");
/// assert_eq!(holidays.len(), 21);
/// ```
pub fn holidays<T: DateLike>(start_date: T, end_date: T) -> Vec<(NaiveDate, String)> {
    between(start_date, end_date)
}

/// 指定した期間の祝日を取得します.
/// 祝日が存在しない場合は空のリストを返します.
/// holidays のエイリアスです.
///
/// # Arguments
///
/// * `start_date` - 開始日(含む)
/// * `end_date` - 終了日(含む)
///
/// # Returns
///
/// 祝日のリストを返します.
///
/// # Examples
/// ```
/// use yasumi::between;
///
/// let holidays = between("2024-01-01", "2024-12-31");
/// assert_eq!(holidays.len(), 21);
/// ```
pub fn between<T: DateLike>(start_date: T, end_date: T) -> Vec<(NaiveDate, String)> {
    let start_date = start_date.date().unwrap();
    let end_date = end_date.date().unwrap();

    let mut holidays = vec![];

    let mut date = start_date;
    while date <= end_date {
        if let Some(name) = is_holiday_name(date) {
            holidays.push((date, name));
        }
        date = date.succ_opt().unwrap();
    }

    holidays
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2024/09/13", false)] // 平日(金曜日)
    #[case("2024/09/14", true)] // 土曜日
    #[case("2024/09/15", true)] // 日曜日
    #[case("2024/09/16", true)] // 祝日(敬老の日)
    #[case("2024/09/17", false)] // 平日(火曜日)
    fn test_is_no_workday(#[case] date: &str, #[case] expected: bool) {
        let date = NaiveDate::parse_from_str(date, "%Y/%m/%d").unwrap();
        assert_eq!(is_no_workday(date), expected);
    }

    #[rstest]
    // 1971年
    #[case("1971/01/01", Some("元日".to_string()))]
    #[case("1971/01/15", Some("成人の日".to_string()))]
    #[case("1971/02/11", Some("建国記念の日".to_string()))]
    #[case("1971/03/21", Some("春分の日".to_string()))]
    #[case("1971/04/29", Some("天皇誕生日".to_string()))]
    #[case("1971/05/03", Some("憲法記念日".to_string()))]
    #[case("1971/05/04", Some("国民の休日".to_string()))]
    #[case("1971/05/05", Some("こどもの日".to_string()))]
    #[case("1971/09/15", Some("敬老の日".to_string()))]
    #[case("1971/09/24", Some("秋分の日".to_string()))]
    #[case("1971/10/10", Some("体育の日".to_string()))]
    #[case("1971/11/03", Some("文化の日".to_string()))]
    #[case("1971/11/23", Some("勤労感謝の日".to_string()))]
    // 1988年
    #[case("1988/01/01", Some("元日".to_string()))]
    #[case("1988/01/15", Some("成人の日".to_string()))]
    #[case("1988/02/11", Some("建国記念の日".to_string()))]
    #[case("1988/03/20", Some("春分の日".to_string()))]
    #[case("1988/03/21", Some("春分の日 振替休日".to_string()))]
    #[case("1988/04/29", Some("天皇誕生日".to_string()))]
    #[case("1988/05/03", Some("憲法記念日".to_string()))]
    #[case("1988/05/04", Some("国民の休日".to_string()))]
    #[case("1988/05/05", Some("こどもの日".to_string()))]
    #[case("1988/09/15", Some("敬老の日".to_string()))]
    #[case("1988/09/23", Some("秋分の日".to_string()))]
    #[case("1988/10/10", Some("体育の日".to_string()))]
    #[case("1988/11/03", Some("文化の日".to_string()))]
    #[case("1988/11/23", Some("勤労感謝の日".to_string()))]
    // 1989年
    #[case("1989/01/01", Some("元日".to_string()))]
    #[case("1989/01/02", Some("元日 振替休日".to_string()))]
    #[case("1989/01/15", Some("成人の日".to_string()))]
    #[case("1989/01/16", Some("成人の日 振替休日".to_string()))]
    #[case("1989/02/11", Some("建国記念の日".to_string()))]
    #[case("1989/02/24", Some("昭和天皇の大喪の礼".to_string()))]
    #[case("1989/03/21", Some("春分の日".to_string()))]
    #[case("1989/04/29", Some("みどりの日".to_string()))]
    #[case("1989/05/03", Some("憲法記念日".to_string()))]
    #[case("1989/05/04", Some("国民の休日".to_string()))]
    #[case("1989/05/05", Some("こどもの日".to_string()))]
    #[case("1989/09/15", Some("敬老の日".to_string()))]
    #[case("1989/09/23", Some("秋分の日".to_string()))]
    #[case("1989/10/10", Some("体育の日".to_string()))]
    #[case("1989/11/03", Some("文化の日".to_string()))]
    #[case("1989/11/23", Some("勤労感謝の日".to_string()))]
    #[case("1989/12/23", Some("天皇誕生日".to_string()))]
    // 1992年
    #[case("1992/01/01", Some("元日".to_string()))]
    #[case("1992/01/15", Some("成人の日".to_string()))]
    #[case("1992/02/11", Some("建国記念の日".to_string()))]
    #[case("1992/03/20", Some("春分の日".to_string()))]
    #[case("1992/04/29", Some("みどりの日".to_string()))]
    #[case("1992/05/03", Some("憲法記念日".to_string()))]
    #[case("1992/05/04", Some("憲法記念日 振替休日".to_string()))]
    #[case("1992/05/05", Some("こどもの日".to_string()))]
    #[case("1992/05/06", None)]
    #[case("1992/09/15", Some("敬老の日".to_string()))]
    #[case("1992/09/23", Some("秋分の日".to_string()))]
    #[case("1992/10/10", Some("体育の日".to_string()))]
    #[case("1992/11/03", Some("文化の日".to_string()))]
    #[case("1992/11/23", Some("勤労感謝の日".to_string()))]
    #[case("1992/12/23", Some("天皇誕生日".to_string()))]
    // 1997年
    #[case("1997/01/01", Some("元日".to_string()))]
    #[case("1997/01/15", Some("成人の日".to_string()))]
    #[case("1997/02/11", Some("建国記念の日".to_string()))]
    #[case("1997/03/20", Some("春分の日".to_string()))]
    #[case("1997/04/29", Some("みどりの日".to_string()))]
    #[case("1997/05/03", Some("憲法記念日".to_string()))]
    #[case("1997/05/04", None)]
    #[case("1997/05/05", Some("こどもの日".to_string()))]
    #[case("1997/07/20", Some("海の日".to_string()))]
    #[case("1997/07/21", Some("海の日 振替休日".to_string()))]
    #[case("1997/09/15", Some("敬老の日".to_string()))]
    #[case("1997/09/23", Some("秋分の日".to_string()))]
    #[case("1997/10/10", Some("体育の日".to_string()))]
    #[case("1997/11/03", Some("文化の日".to_string()))]
    #[case("1997/11/23", Some("勤労感謝の日".to_string()))]
    #[case("1997/11/24", Some("勤労感謝の日 振替休日".to_string()))]
    #[case("1997/12/23", Some("天皇誕生日".to_string()))]
    // 1998年
    #[case("1998/01/01", Some("元日".to_string()))]
    #[case("1998/01/15", Some("成人の日".to_string()))]
    #[case("1998/02/11", Some("建国記念の日".to_string()))]
    #[case("1998/03/21", Some("春分の日".to_string()))]
    #[case("1998/04/29", Some("みどりの日".to_string()))]
    #[case("1998/05/03", Some("憲法記念日".to_string()))]
    #[case("1998/05/04", Some("憲法記念日 振替休日".to_string()))]
    #[case("1998/05/05", Some("こどもの日".to_string()))]
    #[case("1998/05/06", None)]
    #[case("1998/07/20", Some("海の日".to_string()))]
    #[case("1998/09/15", Some("敬老の日".to_string()))]
    #[case("1998/09/23", Some("秋分の日".to_string()))]
    #[case("1998/10/10", Some("体育の日".to_string()))]
    #[case("1998/11/03", Some("文化の日".to_string()))]
    #[case("1998/11/23", Some("勤労感謝の日".to_string()))]
    #[case("1998/12/23", Some("天皇誕生日".to_string()))]
    // 1999年
    #[case("1999/01/01", Some("元日".to_string()))]
    #[case("1999/01/15", Some("成人の日".to_string()))]
    #[case("1999/02/11", Some("建国記念の日".to_string()))]
    #[case("1999/03/21", Some("春分の日".to_string()))]
    #[case("1999/03/22", Some("春分の日 振替休日".to_string()))]
    #[case("1999/04/29", Some("みどりの日".to_string()))]
    #[case("1999/05/03", Some("憲法記念日".to_string()))]
    #[case("1999/05/04", Some("国民の休日".to_string()))]
    #[case("1999/05/05", Some("こどもの日".to_string()))]
    #[case("1999/07/20", Some("海の日".to_string()))]
    #[case("1999/09/15", Some("敬老の日".to_string()))]
    #[case("1999/09/23", Some("秋分の日".to_string()))]
    #[case("1999/10/10", Some("体育の日".to_string()))]
    #[case("1999/10/11", Some("体育の日 振替休日".to_string()))]
    #[case("1999/11/03", Some("文化の日".to_string()))]
    #[case("1999/11/23", Some("勤労感謝の日".to_string()))]
    #[case("1999/12/23", Some("天皇誕生日".to_string()))]
    // 2000年
    #[case("2000/01/01", Some("元日".to_string()))]
    #[case("2000/01/10", Some("成人の日".to_string()))]
    #[case("2000/02/11", Some("建国記念の日".to_string()))]
    #[case("2000/03/20", Some("春分の日".to_string()))]
    #[case("2000/04/29", Some("みどりの日".to_string()))]
    #[case("2000/05/03", Some("憲法記念日".to_string()))]
    #[case("2000/05/04", Some("国民の休日".to_string()))]
    #[case("2000/05/05", Some("こどもの日".to_string()))]
    #[case("2000/07/20", Some("海の日".to_string()))]
    #[case("2000/09/15", Some("敬老の日".to_string()))]
    #[case("2000/09/23", Some("秋分の日".to_string()))]
    #[case("2000/10/09", Some("体育の日".to_string()))]
    #[case("2000/11/03", Some("文化の日".to_string()))]
    #[case("2000/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2000/12/23", Some("天皇誕生日".to_string()))]
    // 2001年
    #[case("2001/01/01", Some("元日".to_string()))]
    #[case("2001/01/08", Some("成人の日".to_string()))]
    #[case("2001/02/11", Some("建国記念の日".to_string()))]
    #[case("2001/02/12", Some("建国記念の日 振替休日".to_string()))]
    #[case("2001/03/20", Some("春分の日".to_string()))]
    #[case("2001/04/29", Some("みどりの日".to_string()))]
    #[case("2001/04/30", Some("みどりの日 振替休日".to_string()))]
    #[case("2001/05/03", Some("憲法記念日".to_string()))]
    #[case("2001/05/04", Some("国民の休日".to_string()))]
    #[case("2001/05/05", Some("こどもの日".to_string()))]
    #[case("2001/07/20", Some("海の日".to_string()))]
    #[case("2001/09/15", Some("敬老の日".to_string()))]
    #[case("2001/09/23", Some("秋分の日".to_string()))]
    #[case("2001/09/24", Some("秋分の日 振替休日".to_string()))]
    #[case("2001/10/08", Some("体育の日".to_string()))]
    #[case("2001/11/03", Some("文化の日".to_string()))]
    #[case("2001/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2001/12/23", Some("天皇誕生日".to_string()))]
    #[case("2001/12/24", Some("天皇誕生日 振替休日".to_string()))]
    // 2002年
    #[case("2002/01/01", Some("元日".to_string()))]
    #[case("2002/01/14", Some("成人の日".to_string()))]
    #[case("2002/02/11", Some("建国記念の日".to_string()))]
    #[case("2002/03/21", Some("春分の日".to_string()))]
    #[case("2002/04/29", Some("みどりの日".to_string()))]
    #[case("2002/05/03", Some("憲法記念日".to_string()))]
    #[case("2002/05/04", Some("国民の休日".to_string()))]
    #[case("2002/05/05", Some("こどもの日".to_string()))]
    #[case("2002/05/06", Some("こどもの日 振替休日".to_string()))]
    #[case("2002/07/20", Some("海の日".to_string()))]
    #[case("2002/09/15", Some("敬老の日".to_string()))]
    #[case("2002/09/16", Some("敬老の日 振替休日".to_string()))]
    #[case("2002/09/23", Some("秋分の日".to_string()))]
    #[case("2002/10/14", Some("体育の日".to_string()))]
    #[case("2002/11/03", Some("文化の日".to_string()))]
    #[case("2002/11/04", Some("文化の日 振替休日".to_string()))]
    #[case("2002/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2002/12/23", Some("天皇誕生日".to_string()))]
    // 2003年
    #[case("2003/01/01", Some("元日".to_string()))]
    #[case("2003/01/13", Some("成人の日".to_string()))]
    #[case("2003/02/11", Some("建国記念の日".to_string()))]
    #[case("2003/03/21", Some("春分の日".to_string()))]
    #[case("2003/04/29", Some("みどりの日".to_string()))]
    #[case("2003/05/03", Some("憲法記念日".to_string()))]
    #[case("2003/05/05", Some("こどもの日".to_string()))]
    #[case("2003/07/21", Some("海の日".to_string()))]
    #[case("2003/09/15", Some("敬老の日".to_string()))]
    #[case("2003/09/23", Some("秋分の日".to_string()))]
    #[case("2003/10/13", Some("体育の日".to_string()))]
    #[case("2003/11/03", Some("文化の日".to_string()))]
    #[case("2003/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2003/11/24", Some("勤労感謝の日 振替休日".to_string()))]
    #[case("2003/12/23", Some("天皇誕生日".to_string()))]
    // 2004年
    #[case("2004/01/01", Some("元日".to_string()))]
    #[case("2004/01/12", Some("成人の日".to_string()))]
    #[case("2004/02/11", Some("建国記念の日".to_string()))]
    #[case("2004/03/20", Some("春分の日".to_string()))]
    #[case("2004/04/29", Some("みどりの日".to_string()))]
    #[case("2004/05/03", Some("憲法記念日".to_string()))]
    #[case("2004/05/04", Some("国民の休日".to_string()))]
    #[case("2004/05/05", Some("こどもの日".to_string()))]
    #[case("2004/07/19", Some("海の日".to_string()))]
    #[case("2004/09/20", Some("敬老の日".to_string()))]
    #[case("2004/09/23", Some("秋分の日".to_string()))]
    #[case("2004/10/11", Some("体育の日".to_string()))]
    #[case("2004/11/03", Some("文化の日".to_string()))]
    #[case("2004/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2004/12/23", Some("天皇誕生日".to_string()))]
    // 2005年
    #[case("2005/01/01", Some("元日".to_string()))]
    #[case("2005/01/10", Some("成人の日".to_string()))]
    #[case("2005/02/11", Some("建国記念の日".to_string()))]
    #[case("2005/03/20", Some("春分の日".to_string()))]
    #[case("2005/03/21", Some("春分の日 振替休日".to_string()))]
    #[case("2005/04/29", Some("みどりの日".to_string()))]
    #[case("2005/05/03", Some("憲法記念日".to_string()))]
    #[case("2005/05/04", Some("国民の休日".to_string()))]
    #[case("2005/05/05", Some("こどもの日".to_string()))]
    #[case("2005/07/18", Some("海の日".to_string()))]
    #[case("2005/09/19", Some("敬老の日".to_string()))]
    #[case("2005/09/23", Some("秋分の日".to_string()))]
    #[case("2005/10/10", Some("体育の日".to_string()))]
    #[case("2005/11/03", Some("文化の日".to_string()))]
    #[case("2005/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2005/12/23", Some("天皇誕生日".to_string()))]
    // 2006年
    #[case("2006/01/01", Some("元日".to_string()))]
    #[case("2006/01/02", Some("元日 振替休日".to_string()))]
    #[case("2006/01/09", Some("成人の日".to_string()))]
    #[case("2006/02/11", Some("建国記念の日".to_string()))]
    #[case("2006/03/21", Some("春分の日".to_string()))]
    #[case("2006/04/29", Some("みどりの日".to_string()))]
    #[case("2006/05/03", Some("憲法記念日".to_string()))]
    #[case("2006/05/04", Some("国民の休日".to_string()))]
    #[case("2006/05/05", Some("こどもの日".to_string()))]
    #[case("2006/07/17", Some("海の日".to_string()))]
    #[case("2006/09/18", Some("敬老の日".to_string()))]
    #[case("2006/09/23", Some("秋分の日".to_string()))]
    #[case("2006/10/09", Some("体育の日".to_string()))]
    #[case("2006/11/03", Some("文化の日".to_string()))]
    #[case("2006/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2006/12/23", Some("天皇誕生日".to_string()))]
    // 2007年
    #[case("2007/01/01", Some("元日".to_string()))]
    #[case("2007/01/08", Some("成人の日".to_string()))]
    #[case("2007/02/11", Some("建国記念の日".to_string()))]
    #[case("2007/02/12", Some("建国記念の日 振替休日".to_string()))]
    #[case("2007/03/21", Some("春分の日".to_string()))]
    #[case("2007/04/29", Some("昭和の日".to_string()))]
    #[case("2007/04/30", Some("昭和の日 振替休日".to_string()))]
    #[case("2007/05/03", Some("憲法記念日".to_string()))]
    #[case("2007/05/04", Some("みどりの日".to_string()))]
    #[case("2007/05/05", Some("こどもの日".to_string()))]
    #[case("2007/07/16", Some("海の日".to_string()))]
    #[case("2007/09/17", Some("敬老の日".to_string()))]
    #[case("2007/09/23", Some("秋分の日".to_string()))]
    #[case("2007/09/24", Some("秋分の日 振替休日".to_string()))]
    #[case("2007/10/08", Some("体育の日".to_string()))]
    #[case("2007/11/03", Some("文化の日".to_string()))]
    #[case("2007/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2007/12/23", Some("天皇誕生日".to_string()))]
    #[case("2007/12/24", Some("天皇誕生日 振替休日".to_string()))]
    // 2008年
    #[case("2008/01/01", Some("元日".to_string()))]
    #[case("2008/01/14", Some("成人の日".to_string()))]
    #[case("2008/02/11", Some("建国記念の日".to_string()))]
    #[case("2008/03/20", Some("春分の日".to_string()))]
    #[case("2008/04/29", Some("昭和の日".to_string()))]
    #[case("2008/05/03", Some("憲法記念日".to_string()))]
    #[case("2008/05/04", Some("みどりの日".to_string()))]
    #[case("2008/05/05", Some("こどもの日".to_string()))]
    #[case("2008/05/06", Some("みどりの日 振替休日".to_string()))]
    #[case("2008/07/21", Some("海の日".to_string()))]
    #[case("2008/09/15", Some("敬老の日".to_string()))]
    #[case("2008/09/23", Some("秋分の日".to_string()))]
    #[case("2008/10/13", Some("体育の日".to_string()))]
    #[case("2008/11/03", Some("文化の日".to_string()))]
    #[case("2008/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2008/11/24", Some("勤労感謝の日 振替休日".to_string()))]
    #[case("2008/12/23", Some("天皇誕生日".to_string()))]
    // 2009年
    #[case("2009/01/01", Some("元日".to_string()))]
    #[case("2009/01/12", Some("成人の日".to_string()))]
    #[case("2009/02/11", Some("建国記念の日".to_string()))]
    #[case("2009/03/20", Some("春分の日".to_string()))]
    #[case("2009/04/29", Some("昭和の日".to_string()))]
    #[case("2009/05/03", Some("憲法記念日".to_string()))]
    #[case("2009/05/04", Some("みどりの日".to_string()))]
    #[case("2009/05/05", Some("こどもの日".to_string()))]
    #[case("2009/05/06", Some("憲法記念日 振替休日".to_string()))]
    #[case("2009/07/20", Some("海の日".to_string()))]
    #[case("2009/09/21", Some("敬老の日".to_string()))]
    #[case("2009/09/22", Some("国民の休日".to_string()))]
    #[case("2009/09/23", Some("秋分の日".to_string()))]
    #[case("2009/10/12", Some("体育の日".to_string()))]
    #[case("2009/11/03", Some("文化の日".to_string()))]
    #[case("2009/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2009/12/23", Some("天皇誕生日".to_string()))]
    // 2010年
    #[case("2010/01/01", Some("元日".to_string()))]
    #[case("2010/01/11", Some("成人の日".to_string()))]
    #[case("2010/02/11", Some("建国記念の日".to_string()))]
    #[case("2010/03/21", Some("春分の日".to_string()))]
    #[case("2010/03/22", Some("春分の日 振替休日".to_string()))]
    #[case("2010/04/29", Some("昭和の日".to_string()))]
    #[case("2010/05/03", Some("憲法記念日".to_string()))]
    #[case("2010/05/04", Some("みどりの日".to_string()))]
    #[case("2010/05/05", Some("こどもの日".to_string()))]
    #[case("2010/07/19", Some("海の日".to_string()))]
    #[case("2010/09/20", Some("敬老の日".to_string()))]
    #[case("2010/09/23", Some("秋分の日".to_string()))]
    #[case("2010/10/11", Some("体育の日".to_string()))]
    #[case("2010/11/03", Some("文化の日".to_string()))]
    #[case("2010/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2010/12/23", Some("天皇誕生日".to_string()))]
    // 2011年
    #[case("2011/01/01", Some("元日".to_string()))]
    #[case("2011/01/10", Some("成人の日".to_string()))]
    #[case("2011/02/11", Some("建国記念の日".to_string()))]
    #[case("2011/03/21", Some("春分の日".to_string()))]
    #[case("2011/04/29", Some("昭和の日".to_string()))]
    #[case("2011/05/03", Some("憲法記念日".to_string()))]
    #[case("2011/05/04", Some("みどりの日".to_string()))]
    #[case("2011/05/05", Some("こどもの日".to_string()))]
    #[case("2011/07/18", Some("海の日".to_string()))]
    #[case("2011/09/19", Some("敬老の日".to_string()))]
    #[case("2011/09/23", Some("秋分の日".to_string()))]
    #[case("2011/10/10", Some("体育の日".to_string()))]
    #[case("2011/11/03", Some("文化の日".to_string()))]
    #[case("2011/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2011/12/23", Some("天皇誕生日".to_string()))]
    // 2012年
    #[case("2012/01/01", Some("元日".to_string()))]
    #[case("2012/01/02", Some("元日 振替休日".to_string()))]
    #[case("2012/01/09", Some("成人の日".to_string()))]
    #[case("2012/02/11", Some("建国記念の日".to_string()))]
    #[case("2012/03/20", Some("春分の日".to_string()))]
    #[case("2012/04/29", Some("昭和の日".to_string()))]
    #[case("2012/04/30", Some("昭和の日 振替休日".to_string()))]
    #[case("2012/05/03", Some("憲法記念日".to_string()))]
    #[case("2012/05/04", Some("みどりの日".to_string()))]
    #[case("2012/05/05", Some("こどもの日".to_string()))]
    #[case("2012/07/16", Some("海の日".to_string()))]
    #[case("2012/09/17", Some("敬老の日".to_string()))]
    #[case("2012/09/22", Some("秋分の日".to_string()))]
    #[case("2012/10/08", Some("体育の日".to_string()))]
    #[case("2012/11/03", Some("文化の日".to_string()))]
    #[case("2012/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2012/12/23", Some("天皇誕生日".to_string()))]
    #[case("2012/12/24", Some("天皇誕生日 振替休日".to_string()))]
    // 2013年
    #[case("2013/01/01", Some("元日".to_string()))]
    #[case("2013/01/14", Some("成人の日".to_string()))]
    #[case("2013/02/11", Some("建国記念の日".to_string()))]
    #[case("2013/03/20", Some("春分の日".to_string()))]
    #[case("2013/04/29", Some("昭和の日".to_string()))]
    #[case("2013/05/03", Some("憲法記念日".to_string()))]
    #[case("2013/05/04", Some("みどりの日".to_string()))]
    #[case("2013/05/05", Some("こどもの日".to_string()))]
    #[case("2013/05/06", Some("こどもの日 振替休日".to_string()))]
    #[case("2013/07/15", Some("海の日".to_string()))]
    #[case("2013/09/16", Some("敬老の日".to_string()))]
    #[case("2013/09/23", Some("秋分の日".to_string()))]
    #[case("2013/10/14", Some("体育の日".to_string()))]
    #[case("2013/11/03", Some("文化の日".to_string()))]
    #[case("2013/11/04", Some("文化の日 振替休日".to_string()))]
    #[case("2013/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2013/12/23", Some("天皇誕生日".to_string()))]
    // 2014年
    #[case("2014/01/01", Some("元日".to_string()))]
    #[case("2014/01/13", Some("成人の日".to_string()))]
    #[case("2014/02/11", Some("建国記念の日".to_string()))]
    #[case("2014/03/21", Some("春分の日".to_string()))]
    #[case("2014/04/29", Some("昭和の日".to_string()))]
    #[case("2014/05/03", Some("憲法記念日".to_string()))]
    #[case("2014/05/04", Some("みどりの日".to_string()))]
    #[case("2014/05/05", Some("こどもの日".to_string()))]
    #[case("2014/05/06", Some("みどりの日 振替休日".to_string()))]
    #[case("2014/07/21", Some("海の日".to_string()))]
    #[case("2014/09/15", Some("敬老の日".to_string()))]
    #[case("2014/09/23", Some("秋分の日".to_string()))]
    #[case("2014/10/13", Some("体育の日".to_string()))]
    #[case("2014/11/03", Some("文化の日".to_string()))]
    #[case("2014/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2014/11/24", Some("勤労感謝の日 振替休日".to_string()))]
    #[case("2014/12/23", Some("天皇誕生日".to_string()))]
    // 2015年
    #[case("2015/01/01", Some("元日".to_string()))]
    #[case("2015/01/12", Some("成人の日".to_string()))]
    #[case("2015/02/11", Some("建国記念の日".to_string()))]
    #[case("2015/03/21", Some("春分の日".to_string()))]
    #[case("2015/04/29", Some("昭和の日".to_string()))]
    #[case("2015/05/03", Some("憲法記念日".to_string()))]
    #[case("2015/05/04", Some("みどりの日".to_string()))]
    #[case("2015/05/05", Some("こどもの日".to_string()))]
    #[case("2015/05/06", Some("憲法記念日 振替休日".to_string()))]
    #[case("2015/07/20", Some("海の日".to_string()))]
    #[case("2015/09/21", Some("敬老の日".to_string()))]
    #[case("2015/09/22", Some("国民の休日".to_string()))]
    #[case("2015/09/23", Some("秋分の日".to_string()))]
    #[case("2015/10/12", Some("体育の日".to_string()))]
    #[case("2015/11/03", Some("文化の日".to_string()))]
    #[case("2015/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2015/12/23", Some("天皇誕生日".to_string()))]
    // 2016年
    #[case("2016/01/01", Some("元日".to_string()))]
    #[case("2016/01/11", Some("成人の日".to_string()))]
    #[case("2016/02/11", Some("建国記念の日".to_string()))]
    #[case("2016/03/20", Some("春分の日".to_string()))]
    #[case("2016/03/21", Some("春分の日 振替休日".to_string()))]
    #[case("2016/04/29", Some("昭和の日".to_string()))]
    #[case("2016/05/03", Some("憲法記念日".to_string()))]
    #[case("2016/05/04", Some("みどりの日".to_string()))]
    #[case("2016/05/05", Some("こどもの日".to_string()))]
    #[case("2016/07/18", Some("海の日".to_string()))]
    #[case("2016/08/11", Some("山の日".to_string()))]
    #[case("2016/09/19", Some("敬老の日".to_string()))]
    #[case("2016/09/22", Some("秋分の日".to_string()))]
    #[case("2016/10/10", Some("体育の日".to_string()))]
    #[case("2016/11/03", Some("文化の日".to_string()))]
    #[case("2016/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2016/12/23", Some("天皇誕生日".to_string()))]
    // 2017年
    #[case("2017/01/01", Some("元日".to_string()))]
    #[case("2017/01/02", Some("元日 振替休日".to_string()))]
    #[case("2017/01/09", Some("成人の日".to_string()))]
    #[case("2017/02/11", Some("建国記念の日".to_string()))]
    #[case("2017/03/20", Some("春分の日".to_string()))]
    #[case("2017/04/29", Some("昭和の日".to_string()))]
    #[case("2017/05/03", Some("憲法記念日".to_string()))]
    #[case("2017/05/04", Some("みどりの日".to_string()))]
    #[case("2017/05/05", Some("こどもの日".to_string()))]
    #[case("2017/07/17", Some("海の日".to_string()))]
    #[case("2017/08/11", Some("山の日".to_string()))]
    #[case("2017/09/18", Some("敬老の日".to_string()))]
    #[case("2017/09/23", Some("秋分の日".to_string()))]
    #[case("2017/10/09", Some("体育の日".to_string()))]
    #[case("2017/11/03", Some("文化の日".to_string()))]
    #[case("2017/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2017/12/23", Some("天皇誕生日".to_string()))]
    // 2018年
    #[case("2018/01/01", Some("元日".to_string()))]
    #[case("2018/01/08", Some("成人の日".to_string()))]
    #[case("2018/02/11", Some("建国記念の日".to_string()))]
    #[case("2018/02/12", Some("建国記念の日 振替休日".to_string()))]
    #[case("2018/03/21", Some("春分の日".to_string()))]
    #[case("2018/04/29", Some("昭和の日".to_string()))]
    #[case("2018/04/30", Some("昭和の日 振替休日".to_string()))]
    #[case("2018/05/03", Some("憲法記念日".to_string()))]
    #[case("2018/05/04", Some("みどりの日".to_string()))]
    #[case("2018/05/05", Some("こどもの日".to_string()))]
    #[case("2018/07/16", Some("海の日".to_string()))]
    #[case("2018/08/11", Some("山の日".to_string()))]
    #[case("2018/09/17", Some("敬老の日".to_string()))]
    #[case("2018/09/23", Some("秋分の日".to_string()))]
    #[case("2018/09/24", Some("秋分の日 振替休日".to_string()))]
    #[case("2018/10/8", Some("体育の日".to_string()))]
    #[case("2018/11/03", Some("文化の日".to_string()))]
    #[case("2018/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2018/12/23", Some("天皇誕生日".to_string()))]
    #[case("2018/12/24", Some("天皇誕生日 振替休日".to_string()))]
    // 2019年
    #[case("2019/01/01", Some("元日".to_string()))]
    #[case("2019/01/14", Some("成人の日".to_string()))]
    #[case("2019/02/11", Some("建国記念の日".to_string()))]
    #[case("2019/03/21", Some("春分の日".to_string()))]
    #[case("2019/04/29", Some("昭和の日".to_string()))]
    #[case("2019/04/30", Some("国民の休日".to_string()))]
    #[case("2019/05/01", Some("天皇の即位の日".to_string()))]
    #[case("2019/05/02", Some("国民の休日".to_string()))]
    #[case("2019/05/03", Some("憲法記念日".to_string()))]
    #[case("2019/05/04", Some("みどりの日".to_string()))]
    #[case("2019/05/05", Some("こどもの日".to_string()))]
    #[case("2019/05/06", Some("こどもの日 振替休日".to_string()))]
    #[case("2019/07/15", Some("海の日".to_string()))]
    #[case("2019/08/11", Some("山の日".to_string()))]
    #[case("2019/08/12", Some("山の日 振替休日".to_string()))]
    #[case("2019/09/16", Some("敬老の日".to_string()))]
    #[case("2019/09/23", Some("秋分の日".to_string()))]
    #[case("2019/10/14", Some("体育の日".to_string()))]
    #[case("2019/10/22", Some("即位礼正殿の儀".to_string()))]
    #[case("2019/11/03", Some("文化の日".to_string()))]
    #[case("2019/11/04", Some("文化の日 振替休日".to_string()))]
    #[case("2019/11/23", Some("勤労感謝の日".to_string()))]
    // 2020年
    #[case("2020/01/01", Some("元日".to_string()))]
    #[case("2020/01/13", Some("成人の日".to_string()))]
    #[case("2020/02/11", Some("建国記念の日".to_string()))]
    #[case("2020/02/23", Some("天皇誕生日".to_string()))]
    #[case("2020/02/24", Some("天皇誕生日 振替休日".to_string()))]
    #[case("2020/03/20", Some("春分の日".to_string()))]
    #[case("2020/04/29", Some("昭和の日".to_string()))]
    #[case("2020/05/03", Some("憲法記念日".to_string()))]
    #[case("2020/05/04", Some("みどりの日".to_string()))]
    #[case("2020/05/05", Some("こどもの日".to_string()))]
    #[case("2020/05/06", Some("憲法記念日 振替休日".to_string()))]
    #[case("2020/07/23", Some("海の日".to_string()))]
    #[case("2020/07/24", Some("スポーツの日".to_string()))]
    #[case("2020/08/10", Some("山の日".to_string()))]
    #[case("2020/09/21", Some("敬老の日".to_string()))]
    #[case("2020/09/22", Some("秋分の日".to_string()))]
    #[case("2020/11/03", Some("文化の日".to_string()))]
    #[case("2020/11/23", Some("勤労感謝の日".to_string()))]
    // 2021年
    #[case("2021/01/01", Some("元日".to_string()))]
    #[case("2021/01/11", Some("成人の日".to_string()))]
    #[case("2021/02/11", Some("建国記念の日".to_string()))]
    #[case("2021/02/23", Some("天皇誕生日".to_string()))]
    #[case("2021/03/20", Some("春分の日".to_string()))]
    #[case("2021/04/29", Some("昭和の日".to_string()))]
    #[case("2021/05/03", Some("憲法記念日".to_string()))]
    #[case("2021/05/04", Some("みどりの日".to_string()))]
    #[case("2021/05/05", Some("こどもの日".to_string()))]
    #[case("2021/07/22", Some("海の日".to_string()))]
    #[case("2021/07/23", Some("スポーツの日".to_string()))]
    #[case("2021/08/08", Some("山の日".to_string()))]
    #[case("2021/08/09", Some("山の日 振替休日".to_string()))]
    #[case("2021/09/20", Some("敬老の日".to_string()))]
    #[case("2021/09/23", Some("秋分の日".to_string()))]
    #[case("2021/11/03", Some("文化の日".to_string()))]
    #[case("2021/11/23", Some("勤労感謝の日".to_string()))]
    // 2022年
    #[case("2022/01/01", Some("元日".to_string()))]
    #[case("2022/01/10", Some("成人の日".to_string()))]
    #[case("2022/02/11", Some("建国記念の日".to_string()))]
    #[case("2022/02/23", Some("天皇誕生日".to_string()))]
    #[case("2022/03/21", Some("春分の日".to_string()))]
    #[case("2022/04/29", Some("昭和の日".to_string()))]
    #[case("2022/05/03", Some("憲法記念日".to_string()))]
    #[case("2022/05/04", Some("みどりの日".to_string()))]
    #[case("2022/05/05", Some("こどもの日".to_string()))]
    #[case("2022/07/18", Some("海の日".to_string()))]
    #[case("2022/08/11", Some("山の日".to_string()))]
    #[case("2022/09/19", Some("敬老の日".to_string()))]
    #[case("2022/09/23", Some("秋分の日".to_string()))]
    #[case("2022/10/10", Some("スポーツの日".to_string()))]
    #[case("2022/11/03", Some("文化の日".to_string()))]
    #[case("2022/11/23", Some("勤労感謝の日".to_string()))]
    // 2023年
    #[case("2023/01/01", Some("元日".to_string()))]
    #[case("2023/01/02", Some("元日 振替休日".to_string()))]
    #[case("2023/01/09", Some("成人の日".to_string()))]
    #[case("2023/02/11", Some("建国記念の日".to_string()))]
    #[case("2023/02/23", Some("天皇誕生日".to_string()))]
    #[case("2023/03/21", Some("春分の日".to_string()))]
    #[case("2023/04/29", Some("昭和の日".to_string()))]
    #[case("2023/05/03", Some("憲法記念日".to_string()))]
    #[case("2023/05/04", Some("みどりの日".to_string()))]
    #[case("2023/05/05", Some("こどもの日".to_string()))]
    #[case("2023/07/17", Some("海の日".to_string()))]
    #[case("2023/08/11", Some("山の日".to_string()))]
    #[case("2023/09/18", Some("敬老の日".to_string()))]
    #[case("2023/09/23", Some("秋分の日".to_string()))]
    #[case("2023/10/09", Some("スポーツの日".to_string()))]
    #[case("2023/11/03", Some("文化の日".to_string()))]
    #[case("2023/11/23", Some("勤労感謝の日".to_string()))]
    // 2024年
    #[case("2024/01/01", Some("元日".to_string()))]
    #[case("2024/01/08", Some("成人の日".to_string()))]
    #[case("2024/02/11", Some("建国記念の日".to_string()))]
    #[case("2024/02/12", Some("建国記念の日 振替休日".to_string()))]
    #[case("2024/02/23", Some("天皇誕生日".to_string()))]
    #[case("2024/03/20", Some("春分の日".to_string()))]
    #[case("2024/04/29", Some("昭和の日".to_string()))]
    #[case("2024/05/03", Some("憲法記念日".to_string()))]
    #[case("2024/05/04", Some("みどりの日".to_string()))]
    #[case("2024/05/05", Some("こどもの日".to_string()))]
    #[case("2024/05/06", Some("こどもの日 振替休日".to_string()))]
    #[case("2024/07/15", Some("海の日".to_string()))]
    #[case("2024/08/11", Some("山の日".to_string()))]
    #[case("2024/08/12", Some("山の日 振替休日".to_string()))]
    #[case("2024/09/16", Some("敬老の日".to_string()))]
    #[case("2024/09/22", Some("秋分の日".to_string()))]
    #[case("2024/09/23", Some("秋分の日 振替休日".to_string()))]
    #[case("2024/10/14", Some("スポーツの日".to_string()))]
    #[case("2024/11/03", Some("文化の日".to_string()))]
    #[case("2024/11/04", Some("文化の日 振替休日".to_string()))]
    #[case("2024/11/23", Some("勤労感謝の日".to_string()))]
    // 2025年
    #[case("2025/01/01", Some("元日".to_string()))]
    #[case("2025/01/13", Some("成人の日".to_string()))]
    #[case("2025/02/11", Some("建国記念の日".to_string()))]
    #[case("2025/02/23", Some("天皇誕生日".to_string()))]
    #[case("2025/02/24", Some("天皇誕生日 振替休日".to_string()))]
    #[case("2025/03/20", Some("春分の日".to_string()))]
    #[case("2025/04/29", Some("昭和の日".to_string()))]
    #[case("2025/05/03", Some("憲法記念日".to_string()))]
    #[case("2025/05/04", Some("みどりの日".to_string()))]
    #[case("2025/05/05", Some("こどもの日".to_string()))]
    #[case("2025/05/06", Some("みどりの日 振替休日".to_string()))]
    #[case("2025/07/21", Some("海の日".to_string()))]
    #[case("2025/08/11", Some("山の日".to_string()))]
    #[case("2025/09/15", Some("敬老の日".to_string()))]
    #[case("2025/09/23", Some("秋分の日".to_string()))]
    #[case("2025/10/13", Some("スポーツの日".to_string()))]
    #[case("2025/11/03", Some("文化の日".to_string()))]
    #[case("2025/11/23", Some("勤労感謝の日".to_string()))]
    #[case("2025/11/24", Some("勤労感謝の日 振替休日".to_string()))]
    fn test_is_holiday_name(#[case] date: &str, #[case] expected: Option<String>) {
        let result = is_holiday_name(date);
        assert_eq!(result, expected);
    }

    #[rstest]
    // 1971年
    #[case(1971, 1, 2)]
    #[case(1971, 2, 1)]
    #[case(1971, 3, 1)]
    #[case(1971, 4, 1)]
    #[case(1971, 5, 3)]
    #[case(1971, 6, 0)]
    #[case(1971, 7, 0)]
    #[case(1971, 8, 0)]
    #[case(1971, 9, 2)]
    #[case(1971, 10, 1)]
    #[case(1971, 11, 2)]
    #[case(1971, 12, 0)]
    // 1988年
    #[case(1988, 1, 2)]
    #[case(1988, 2, 1)]
    #[case(1988, 3, 2)]
    #[case(1988, 4, 1)]
    #[case(1988, 5, 3)]
    #[case(1988, 6, 0)]
    #[case(1988, 7, 0)]
    #[case(1988, 8, 0)]
    #[case(1988, 9, 2)]
    #[case(1988, 10, 1)]
    #[case(1988, 11, 2)]
    #[case(1988, 12, 0)]
    // 1989年
    #[case(1989, 1, 4)]
    #[case(1989, 2, 2)]
    #[case(1989, 3, 1)]
    #[case(1989, 4, 1)]
    #[case(1989, 5, 3)]
    #[case(1989, 6, 0)]
    #[case(1989, 7, 0)]
    #[case(1989, 8, 0)]
    #[case(1989, 9, 2)]
    #[case(1989, 10, 1)]
    #[case(1989, 11, 2)]
    #[case(1989, 12, 1)]
    // 1992年
    #[case(1992, 1, 2)]
    #[case(1992, 2, 1)]
    #[case(1992, 3, 1)]
    #[case(1992, 4, 1)]
    #[case(1992, 5, 3)]
    #[case(1992, 6, 0)]
    #[case(1992, 7, 0)]
    #[case(1992, 8, 0)]
    #[case(1992, 9, 2)]
    #[case(1992, 10, 1)]
    #[case(1992, 11, 2)]
    #[case(1992, 12, 1)]
    // 1997年
    #[case(1997, 1, 2)]
    #[case(1997, 2, 1)]
    #[case(1997, 3, 1)]
    #[case(1997, 4, 1)]
    #[case(1997, 5, 2)]
    #[case(1997, 6, 0)]
    #[case(1997, 7, 2)]
    #[case(1997, 8, 0)]
    #[case(1997, 9, 2)]
    #[case(1997, 10, 1)]
    #[case(1997, 11, 3)]
    #[case(1997, 12, 1)]
    // 1998年
    #[case(1998, 1, 2)]
    #[case(1998, 2, 1)]
    #[case(1998, 3, 1)]
    #[case(1998, 4, 1)]
    #[case(1998, 5, 3)]
    #[case(1998, 6, 0)]
    #[case(1998, 7, 1)]
    #[case(1998, 8, 0)]
    #[case(1998, 9, 2)]
    #[case(1998, 10, 1)]
    #[case(1998, 11, 2)]
    #[case(1998, 12, 1)]
    // 1999年
    #[case(1999, 1, 2)]
    #[case(1999, 2, 1)]
    #[case(1999, 3, 2)]
    #[case(1999, 4, 1)]
    #[case(1999, 5, 3)]
    #[case(1999, 6, 0)]
    #[case(1999, 7, 1)]
    #[case(1999, 8, 0)]
    #[case(1999, 9, 2)]
    #[case(1999, 10, 2)]
    #[case(1999, 11, 2)]
    #[case(1999, 12, 1)]
    // 2000年
    #[case(2000, 1, 2)]
    #[case(2000, 2, 1)]
    #[case(2000, 3, 1)]
    #[case(2000, 4, 1)]
    #[case(2000, 5, 3)]
    #[case(2000, 6, 0)]
    #[case(2000, 7, 1)]
    #[case(2000, 8, 0)]
    #[case(2000, 9, 2)]
    #[case(2000, 10, 1)]
    #[case(2000, 11, 2)]
    #[case(2000, 12, 1)]
    // 2001年
    #[case(2001, 1, 2)]
    #[case(2001, 2, 2)]
    #[case(2001, 3, 1)]
    #[case(2001, 4, 2)]
    #[case(2001, 5, 3)]
    #[case(2001, 6, 0)]
    #[case(2001, 7, 1)]
    #[case(2001, 8, 0)]
    #[case(2001, 9, 3)]
    #[case(2001, 10, 1)]
    #[case(2001, 11, 2)]
    #[case(2001, 12, 2)]
    // 2002年
    #[case(2002, 1, 2)]
    #[case(2002, 2, 1)]
    #[case(2002, 3, 1)]
    #[case(2002, 4, 1)]
    #[case(2002, 5, 4)]
    #[case(2002, 6, 0)]
    #[case(2002, 7, 1)]
    #[case(2002, 8, 0)]
    #[case(2002, 9, 3)]
    #[case(2002, 10, 1)]
    #[case(2002, 11, 3)]
    #[case(2002, 12, 1)]
    // 2003年
    #[case(2003, 1, 2)]
    #[case(2003, 2, 1)]
    #[case(2003, 3, 1)]
    #[case(2003, 4, 1)]
    #[case(2003, 5, 2)]
    #[case(2003, 6, 0)]
    #[case(2003, 7, 1)]
    #[case(2003, 8, 0)]
    #[case(2003, 9, 2)]
    #[case(2003, 10, 1)]
    #[case(2003, 11, 3)]
    #[case(2003, 12, 1)]
    // 2004年
    #[case(2004, 1, 2)]
    #[case(2004, 2, 1)]
    #[case(2004, 3, 1)]
    #[case(2004, 4, 1)]
    #[case(2004, 5, 3)]
    #[case(2004, 6, 0)]
    #[case(2004, 7, 1)]
    #[case(2004, 8, 0)]
    #[case(2004, 9, 2)]
    #[case(2004, 10, 1)]
    #[case(2004, 11, 2)]
    #[case(2004, 12, 1)]
    // 2005年
    #[case(2005, 1, 2)]
    #[case(2005, 2, 1)]
    #[case(2005, 3, 2)]
    #[case(2005, 4, 1)]
    #[case(2005, 5, 3)]
    #[case(2005, 6, 0)]
    #[case(2005, 7, 1)]
    #[case(2005, 8, 0)]
    #[case(2005, 9, 2)]
    #[case(2005, 10, 1)]
    #[case(2005, 11, 2)]
    #[case(2005, 12, 1)]
    // 2006年
    #[case(2006, 1, 3)]
    #[case(2006, 2, 1)]
    #[case(2006, 3, 1)]
    #[case(2006, 4, 1)]
    #[case(2006, 5, 3)]
    #[case(2006, 6, 0)]
    #[case(2006, 7, 1)]
    #[case(2006, 8, 0)]
    #[case(2006, 9, 2)]
    #[case(2006, 10, 1)]
    #[case(2006, 11, 2)]
    #[case(2006, 12, 1)]
    // 2007年
    #[case(2007, 1, 2)]
    #[case(2007, 2, 2)]
    #[case(2007, 3, 1)]
    #[case(2007, 4, 2)]
    #[case(2007, 5, 3)]
    #[case(2007, 6, 0)]
    #[case(2007, 7, 1)]
    #[case(2007, 8, 0)]
    #[case(2007, 9, 3)]
    #[case(2007, 10, 1)]
    #[case(2007, 11, 2)]
    #[case(2007, 12, 2)]
    // 2008年
    #[case(2008, 1, 2)]
    #[case(2008, 2, 1)]
    #[case(2008, 3, 1)]
    #[case(2008, 4, 1)]
    #[case(2008, 5, 4)]
    #[case(2008, 6, 0)]
    #[case(2008, 7, 1)]
    #[case(2008, 8, 0)]
    #[case(2008, 9, 2)]
    #[case(2008, 10, 1)]
    #[case(2008, 11, 3)]
    #[case(2008, 12, 1)]
    // 2009年
    #[case(2009, 1, 2)]
    #[case(2009, 2, 1)]
    #[case(2009, 3, 1)]
    #[case(2009, 4, 1)]
    #[case(2009, 5, 4)]
    #[case(2009, 6, 0)]
    #[case(2009, 7, 1)]
    #[case(2009, 8, 0)]
    #[case(2009, 9, 3)]
    #[case(2009, 10, 1)]
    #[case(2009, 11, 2)]
    #[case(2009, 12, 1)]
    // 2010年
    #[case(2010, 1, 2)]
    #[case(2010, 2, 1)]
    #[case(2010, 3, 2)]
    #[case(2010, 4, 1)]
    #[case(2010, 5, 3)]
    #[case(2010, 6, 0)]
    #[case(2010, 7, 1)]
    #[case(2010, 8, 0)]
    #[case(2010, 9, 2)]
    #[case(2010, 10, 1)]
    #[case(2010, 11, 2)]
    #[case(2010, 12, 1)]
    // 2011年
    #[case(2011, 1, 2)]
    #[case(2011, 2, 1)]
    #[case(2011, 3, 1)]
    #[case(2011, 4, 1)]
    #[case(2011, 5, 3)]
    #[case(2011, 6, 0)]
    #[case(2011, 7, 1)]
    #[case(2011, 8, 0)]
    #[case(2011, 9, 2)]
    #[case(2011, 10, 1)]
    #[case(2011, 11, 2)]
    #[case(2011, 12, 1)]
    // 2012年
    #[case(2012, 1, 3)]
    #[case(2012, 2, 1)]
    #[case(2012, 3, 1)]
    #[case(2012, 4, 2)]
    #[case(2012, 5, 3)]
    #[case(2012, 6, 0)]
    #[case(2012, 7, 1)]
    #[case(2012, 8, 0)]
    #[case(2012, 9, 2)]
    #[case(2012, 10, 1)]
    #[case(2012, 11, 2)]
    #[case(2012, 12, 2)]
    // 2013年
    #[case(2013, 1, 2)]
    #[case(2013, 2, 1)]
    #[case(2013, 3, 1)]
    #[case(2013, 4, 1)]
    #[case(2013, 5, 4)]
    #[case(2013, 6, 0)]
    #[case(2013, 7, 1)]
    #[case(2013, 8, 0)]
    #[case(2013, 9, 2)]
    #[case(2013, 10, 1)]
    #[case(2013, 11, 3)]
    #[case(2013, 12, 1)]
    // 2014年
    #[case(2014, 1, 2)]
    #[case(2014, 2, 1)]
    #[case(2014, 3, 1)]
    #[case(2014, 4, 1)]
    #[case(2014, 5, 4)]
    #[case(2014, 6, 0)]
    #[case(2014, 7, 1)]
    #[case(2014, 8, 0)]
    #[case(2014, 9, 2)]
    #[case(2014, 10, 1)]
    #[case(2014, 11, 3)]
    #[case(2014, 12, 1)]
    // 2015年
    #[case(2015, 1, 2)]
    #[case(2015, 2, 1)]
    #[case(2015, 3, 1)]
    #[case(2015, 4, 1)]
    #[case(2015, 5, 4)]
    #[case(2015, 6, 0)]
    #[case(2015, 7, 1)]
    #[case(2015, 8, 0)]
    #[case(2015, 9, 3)]
    #[case(2015, 10, 1)]
    #[case(2015, 11, 2)]
    #[case(2015, 12, 1)]
    // 2016年
    #[case(2016, 1, 2)]
    #[case(2016, 2, 1)]
    #[case(2016, 3, 2)]
    #[case(2016, 4, 1)]
    #[case(2016, 5, 3)]
    #[case(2016, 6, 0)]
    #[case(2016, 7, 1)]
    #[case(2016, 8, 1)]
    #[case(2016, 9, 2)]
    #[case(2016, 10, 1)]
    #[case(2016, 11, 2)]
    #[case(2016, 12, 1)]
    // 2017年
    #[case(2017, 1, 3)]
    #[case(2017, 2, 1)]
    #[case(2017, 3, 1)]
    #[case(2017, 4, 1)]
    #[case(2017, 5, 3)]
    #[case(2017, 6, 0)]
    #[case(2017, 7, 1)]
    #[case(2017, 8, 1)]
    #[case(2017, 9, 2)]
    #[case(2017, 10, 1)]
    #[case(2017, 11, 2)]
    #[case(2017, 12, 1)]
    // 2018年
    #[case(2018, 1, 2)]
    #[case(2018, 2, 2)]
    #[case(2018, 3, 1)]
    #[case(2018, 4, 2)]
    #[case(2018, 5, 3)]
    #[case(2018, 6, 0)]
    #[case(2018, 7, 1)]
    #[case(2018, 8, 1)]
    #[case(2018, 9, 3)]
    #[case(2018, 10, 1)]
    #[case(2018, 11, 2)]
    #[case(2018, 12, 2)]
    // 2019年
    #[case(2019, 1, 2)]
    #[case(2019, 2, 1)]
    #[case(2019, 3, 1)]
    #[case(2019, 4, 2)]
    #[case(2019, 5, 6)]
    #[case(2019, 6, 0)]
    #[case(2019, 7, 1)]
    #[case(2019, 8, 2)]
    #[case(2019, 9, 2)]
    #[case(2019, 10, 2)]
    #[case(2019, 11, 3)]
    #[case(2019, 12, 0)]
    // 2020年
    #[case(2020, 1, 2)]
    #[case(2020, 2, 3)]
    #[case(2020, 3, 1)]
    #[case(2020, 4, 1)]
    #[case(2020, 5, 4)]
    #[case(2020, 6, 0)]
    #[case(2020, 7, 2)]
    #[case(2020, 8, 1)]
    #[case(2020, 9, 2)]
    #[case(2020, 10, 0)]
    #[case(2020, 11, 2)]
    #[case(2020, 12, 0)]
    // // 2021年
    #[case(2021, 1, 2)]
    #[case(2021, 2, 2)]
    #[case(2021, 3, 1)]
    #[case(2021, 4, 1)]
    #[case(2021, 5, 3)]
    #[case(2021, 6, 0)]
    #[case(2021, 7, 2)]
    #[case(2021, 8, 2)]
    #[case(2021, 9, 2)]
    #[case(2021, 10, 0)]
    #[case(2021, 11, 2)]
    #[case(2021, 12, 0)]
    // 2022年
    #[case(2022, 1, 2)]
    #[case(2022, 2, 2)]
    #[case(2022, 3, 1)]
    #[case(2022, 4, 1)]
    #[case(2022, 5, 3)]
    #[case(2022, 6, 0)]
    #[case(2022, 7, 1)]
    #[case(2022, 8, 1)]
    #[case(2022, 9, 2)]
    #[case(2022, 10, 1)]
    #[case(2022, 11, 2)]
    #[case(2022, 12, 0)]
    // 2023年
    #[case(2023, 1, 3)]
    #[case(2023, 2, 2)]
    #[case(2023, 3, 1)]
    #[case(2023, 4, 1)]
    #[case(2023, 5, 3)]
    #[case(2023, 6, 0)]
    #[case(2023, 7, 1)]
    #[case(2023, 8, 1)]
    #[case(2023, 9, 2)]
    #[case(2023, 10, 1)]
    #[case(2023, 11, 2)]
    #[case(2023, 12, 0)]
    // 2024年
    #[case(2024, 1, 2)]
    #[case(2024, 2, 3)]
    #[case(2024, 3, 1)]
    #[case(2024, 4, 1)]
    #[case(2024, 5, 4)]
    #[case(2024, 6, 0)]
    #[case(2024, 7, 1)]
    #[case(2024, 8, 2)]
    #[case(2024, 9, 3)]
    #[case(2024, 10, 1)]
    #[case(2024, 11, 3)]
    #[case(2024, 12, 0)]
    // 2025年
    #[case(2025, 1, 2)]
    #[case(2025, 2, 3)]
    #[case(2025, 3, 1)]
    #[case(2025, 4, 1)]
    #[case(2025, 5, 4)]
    #[case(2025, 6, 0)]
    #[case(2025, 7, 1)]
    #[case(2025, 8, 1)]
    #[case(2025, 9, 2)]
    #[case(2025, 10, 1)]
    #[case(2025, 11, 3)]
    #[case(2025, 12, 0)]
    fn test_month_holidays(#[case] year: i32, #[case] month: u32, #[case] expected: usize) {
        let holidays = month_holidays(year, month);
        assert_eq!(holidays.len(), expected);
    }

    #[rstest]
    // 1971年
    #[case(1971, 13)]
    // 1988年
    #[case(1988, 14)]
    // 1989年
    #[case(1989, 17)]
    // 1992年
    #[case(1992, 14)]
    // 1997年
    #[case(1997, 16)]
    // 1998年
    #[case(1998, 15)]
    // 1999年
    #[case(1999, 17)]
    // 2000年
    #[case(2000, 15)]
    // 2001年
    #[case(2001, 19)]
    // 2002年
    #[case(2002, 18)]
    // 2003年
    #[case(2003, 15)]
    // 2004年
    #[case(2004, 15)]
    // 2005年
    #[case(2005, 16)]
    // 2006年
    #[case(2006, 16)]
    // 2007年
    #[case(2007, 19)]
    // 2008年
    #[case(2008, 17)]
    // 2009年
    #[case(2009, 17)]
    // 2010年
    #[case(2010, 16)]
    // 2011年
    #[case(2011, 15)]
    // 2012年
    #[case(2012, 18)]
    // 2013年
    #[case(2013, 17)]
    // 2014年
    #[case(2014, 17)]
    // 2015年
    #[case(2015, 17)]
    // 2016年
    #[case(2016, 17)]
    // 2017年
    #[case(2017, 17)]
    // 2018年
    #[case(2018, 20)]
    // 2019年
    #[case(2019, 22)]
    // 2020年
    #[case(2020, 18)]
    // 2021年
    #[case(2021, 17)]
    // 2022年
    #[case(2022, 16)]
    // 2023年
    #[case(2023, 17)]
    // 2024年
    #[case(2024, 21)]
    // 2025年
    #[case(2025, 19)]
    fn test_year_holidays(#[case] year: i32, #[case] expected: usize) {
        let holidays = year_holidays(year);
        assert_eq!(holidays.len(), expected);
    }
}
