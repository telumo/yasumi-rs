use chrono::{Datelike, NaiveDate};

pub const HOLIDAYS: [&dyn PublicHoliday; 23] = [
    &NewYearsDay,
    &ComingOfAgeDay,
    &NationalFoundationDay,
    &EmperorsBirthday,
    &VernalEquinoxDay,
    &GreeneryDay,
    &ShowaDay,
    &ConstitutionMemorialDay,
    &ChildrensDay,
    &MarineDay,
    &MountainDay,
    &RespectForTheAgedDay,
    &AutumnalEquinoxDay,
    &HealthAndSportsDay,
    &SportsDay,
    &CultureDay,
    &LaborThanksgivingDay,
    &ImperialEventsTheWeddingCeremonyOfCrownPrinceAkihito,
    &ImperialEventsTheFuneralOfEmperorShowa,
    &ImperialEventsTheCeremonyOfTheEnthronementOfTheEmperor,
    &ImperialEventsTheWeddingCeremonyOfCrownPrinceNaruhito,
    &ImperialEventsTheDayOfTheEmperorsEnthronement,
    &ImperialEventsTheEnthronementCeremony,
];

/// 祝日を表すトレイト
pub trait PublicHoliday {
    fn is_holiday(&self, date: &NaiveDate) -> bool;
    fn name(&self) -> String;
}

/// 元日
pub struct NewYearsDay;
impl PublicHoliday for NewYearsDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.month() == 1 && date.day() == 1
    }
    fn name(&self) -> String {
        "元日".to_string()
    }
}

/// 成人の日
pub struct ComingOfAgeDay;
impl PublicHoliday for ComingOfAgeDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() <= 1999 && date.month() == 1 && date.day() == 15 {
            return true;
        }
        if date.year() >= 2000
            && date.month() == 1
            && date.day() == week_day(date, 2, 1).unwrap().day()
        {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "成人の日".to_string()
    }
}

/// 建国記念の日
pub struct NationalFoundationDay;
impl PublicHoliday for NationalFoundationDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.year() >= 1967 && date.month() == 2 && date.day() == 11
    }
    fn name(&self) -> String {
        "建国記念の日".to_string()
    }
}

/// 天皇誕生日
pub struct EmperorsBirthday;
impl PublicHoliday for EmperorsBirthday {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 1948年から1988年まで: 4月29日
        if date.year() >= 1948 && date.year() <= 1988 && date.month() == 4 && date.day() == 29 {
            return true;
        }
        // 1989年から2018年まで: 12月23日
        // 2019: 国民の祝日に関する法律(昭和23年法律第178号)の一部改正
        if date.year() >= 1989 && date.year() <= 2018 && date.month() == 12 && date.day() == 23 {
            return true;
        }
        // 2019: 国民の祝日に関する法律(昭和23年法律第178号)の一部改正
        if date.year() >= 2020 && date.month() == 2 && date.day() == 23 {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "天皇誕生日".to_string()
    }
}

/// 春分の日
pub struct VernalEquinoxDay;
impl PublicHoliday for VernalEquinoxDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 春分の日の日付を計算する
        // 春季皇霊祭: 1879-1947
        // 春分の日: 1948-
        fn vernal_equinox_day(year: i32) -> u32 {
            // NOTE: http://mt-soft.sakura.ne.jp/kyozai/excel_high/200_jissen_kiso/60_syunbun.htm

            if year <= 1948 {
                return 0;
            }

            let i;
            if (1851..=1899).contains(&year) {
                i = 19.8277;
            } else if (1900..=1979).contains(&year) {
                i = 20.8357;
            } else if (1980..=2099).contains(&year) {
                i = 20.8431;
            } else if (2100..=2150).contains(&year) {
                i = 21.8510;
            } else {
                i = 0.0;
            }

            (i + 0.242194 * (year as f64 - 1980.0) - ((year as f64 - 1980.0) / 4.0).floor()).floor()
                as u32
        }

        if date.month() == 3 && date.day() == vernal_equinox_day(date.year()) {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "春分の日".to_string()
    }
}

/// みどりの日
pub struct GreeneryDay;
impl PublicHoliday for GreeneryDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 1989 && date.year() <= 2006 && date.month() == 4 && date.day() == 29 {
            return true;
        }
        if date.year() >= 2007 && date.month() == 5 && date.day() == 4 {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "みどりの日".to_string()
    }
}

/// 昭和の日
pub struct ShowaDay;
impl PublicHoliday for ShowaDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 2007 && date.month() == 4 && date.day() == 29 {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "昭和の日".to_string()
    }
}

/// 憲法記念日
pub struct ConstitutionMemorialDay;
impl PublicHoliday for ConstitutionMemorialDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.month() == 5 && date.day() == 3
    }
    fn name(&self) -> String {
        "憲法記念日".to_string()
    }
}

/// こどもの日
pub struct ChildrensDay;
impl PublicHoliday for ChildrensDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.month() == 5 && date.day() == 5
    }
    fn name(&self) -> String {
        "こどもの日".to_string()
    }
}

/// 海の日
pub struct MarineDay;
impl PublicHoliday for MarineDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date.year() == 2020 {
            return *date == NaiveDate::from_ymd_opt(2020, 7, 23).unwrap();
        }
        // 2021: 五輪特別措置法改正案
        if date.year() == 2021 {
            return *date == NaiveDate::from_ymd_opt(2021, 7, 22).unwrap();
        }
        if date.year() >= 1996 && date.year() <= 2002 && date.month() == 7 && date.day() == 20 {
            return true;
        }
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        if date.year() >= 2003
            && date.month() == 7
            && date.day() == week_day(date, 3, 1).unwrap().day()
        {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "海の日".to_string()
    }
}

/// 山の日
pub struct MountainDay;
impl PublicHoliday for MountainDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date.year() == 2020 {
            return *date == NaiveDate::from_ymd_opt(2020, 8, 10).unwrap();
        }
        // 2021: 五輪特別措置法改正案
        if date.year() == 2021 {
            return *date == NaiveDate::from_ymd_opt(2021, 8, 8).unwrap();
        }
        // 2016: 国民の祝日に関する法律の一部を改正する法律(平成26年法律第43号)
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        if date.year() >= 2016 && date.month() == 8 && date.day() == 11 {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "山の日".to_string()
    }
}

/// 敬老の日
pub struct RespectForTheAgedDay;
impl PublicHoliday for RespectForTheAgedDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 1966 && date.year() <= 2002 && date.month() == 9 && date.day() == 15 {
            return true;
        }
        if date.year() >= 2003
            && date.month() == 9
            && date.day() == week_day(date, 3, 1).unwrap().day()
        {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "敬老の日".to_string()
    }
}

/// 秋分の日
pub struct AutumnalEquinoxDay;
impl PublicHoliday for AutumnalEquinoxDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 秋分の日の日付を計算する
        // 秋季皇霊祭: 1879-1947
        // 秋分の日: 1948-
        fn autumnal_equinox_day(year: i32) -> u32 {
            // NOTE: http://mt-soft.sakura.ne.jp/kyozai/excel_high/200_jissen_kiso/60_syunbun.htm

            if year <= 1948 {
                return 0;
            }

            let i;
            if (1851..=1899).contains(&year) {
                i = 22.2588;
            } else if (1900..=1979).contains(&year) {
                i = 23.2588;
            } else if (1980..=2099).contains(&year) {
                i = 23.2488;
            } else if (2100..=2150).contains(&year) {
                i = 24.2488;
            } else {
                i = 0.0;
            }

            (i + 0.242194 * (year as f64 - 1980.0) - ((year as f64 - 1980.0) / 4.0).floor()).floor()
                as u32
        }

        if date.month() == 9 && date.day() == autumnal_equinox_day(date.year()) {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "秋分の日".to_string()
    }
}

/// 体育の日
pub struct HealthAndSportsDay;
impl PublicHoliday for HealthAndSportsDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        if date.year() >= 1966 && date.year() <= 1999 && date.month() == 10 && date.day() == 10 {
            return true;
        }
        if date.year() >= 2000
            && date.year() <= 2019
            && date.month() == 10
            && date.day() == week_day(date, 2, 1).unwrap().day()
        {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "体育の日".to_string()
    }
}

/// スポーツの日
pub struct SportsDay;
impl PublicHoliday for SportsDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        // 2020: 国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date.year() == 2020 {
            return *date == NaiveDate::from_ymd_opt(2020, 7, 24).unwrap();
        }
        // 2021: 五輪特別措置法改正案
        if date.year() == 2021 {
            return *date == NaiveDate::from_ymd_opt(2021, 7, 23).unwrap();
        }
        // 2020: 国民の祝日に関する法律の一部を改正する法律(平成30年法律第57号)
        //       国民の祝日に関する法律(昭和23年法律第178号)の特例
        if date.year() >= 2020
            && date.month() == 10
            && date.day() == week_day(date, 2, 1).unwrap().day()
        {
            return true;
        }
        false
    }
    fn name(&self) -> String {
        "スポーツの日".to_string()
    }
}

/// 文化の日
pub struct CultureDay;
impl PublicHoliday for CultureDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.month() == 11 && date.day() == 3
    }
    fn name(&self) -> String {
        "文化の日".to_string()
    }
}

/// 勤労感謝の日
pub struct LaborThanksgivingDay;
impl PublicHoliday for LaborThanksgivingDay {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        date.month() == 11 && date.day() == 23
    }
    fn name(&self) -> String {
        "勤労感謝の日".to_string()
    }
}

/// 皇太子・明仁親王の結婚の儀
pub struct ImperialEventsTheWeddingCeremonyOfCrownPrinceAkihito;
impl PublicHoliday for ImperialEventsTheWeddingCeremonyOfCrownPrinceAkihito {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(1959, 4, 10).unwrap()
    }

    fn name(&self) -> String {
        "皇太子・明仁親王の結婚の儀".to_string()
    }
}

/// 昭和天皇の大喪の礼
pub struct ImperialEventsTheFuneralOfEmperorShowa;
impl PublicHoliday for ImperialEventsTheFuneralOfEmperorShowa {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(1989, 2, 24).unwrap()
    }

    fn name(&self) -> String {
        "昭和天皇の大喪の礼".to_string()
    }
}

/// 即位の礼正殿の儀
pub struct ImperialEventsTheCeremonyOfTheEnthronementOfTheEmperor;
impl PublicHoliday for ImperialEventsTheCeremonyOfTheEnthronementOfTheEmperor {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(1990, 11, 12).unwrap()
    }

    fn name(&self) -> String {
        "即位の礼正殿の儀".to_string()
    }
}

/// 皇太子・皇太子徳仁親王の結婚の儀
pub struct ImperialEventsTheWeddingCeremonyOfCrownPrinceNaruhito;
impl PublicHoliday for ImperialEventsTheWeddingCeremonyOfCrownPrinceNaruhito {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(1993, 6, 9).unwrap()
    }

    fn name(&self) -> String {
        "皇太子・皇太子徳仁親王の結婚の儀".to_string()
    }
}

/// 天皇の即位の日
pub struct ImperialEventsTheDayOfTheEmperorsEnthronement;
impl PublicHoliday for ImperialEventsTheDayOfTheEmperorsEnthronement {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(2019, 5, 1).unwrap()
    }

    fn name(&self) -> String {
        "天皇の即位の日".to_string()
    }
}

/// 即位礼正殿の儀
pub struct ImperialEventsTheEnthronementCeremony;
impl PublicHoliday for ImperialEventsTheEnthronementCeremony {
    fn is_holiday(&self, date: &NaiveDate) -> bool {
        *date == NaiveDate::from_ymd_opt(2019, 10, 22).unwrap()
    }

    fn name(&self) -> String {
        "即位礼正殿の儀".to_string()
    }
}

/// 振替休日を取得する
///
/// # Arguments
///
/// * `date` - 日付
///
/// # Returns
///
/// 振替休日の名前
pub fn substitute_holiday(date: &NaiveDate) -> Option<String> {
    // 1973年(昭和48年)4月12日 - 改正・施行
    if date.year() < 1973 {
        return None;
    }

    // 日曜日に振替休日は存在しない
    if date.weekday().number_from_monday() == 7 {
        return None;
    }

    let mut current_date = *date - chrono::Duration::days(1);
    loop {
        let mut filtered_holiday = HOLIDAYS
            .iter()
            .filter(|holiday| holiday.is_holiday(&current_date));
        if filtered_holiday.clone().count() == 0 {
            return None;
        }
        if current_date.weekday().number_from_monday() == 7 {
            let name = format!("{} 振替休日", filtered_holiday.next().unwrap().name());
            return Some(name);
        }

        current_date -= chrono::Duration::days(1);
    }
}

/// 指定した週の指定した曜日の日付を取得する
/// 週は1から5まで指定可能
/// 曜日は1から7まで指定可能
///
/// # Arguments
///
/// * `date` - 日付
/// * `week` - 週
///
/// # Returns
///
/// 指定した週の指定した曜日の日付
fn week_day(date: &NaiveDate, week: u32, weekday: u32) -> Option<NaiveDate> {
    if !(1..=5).contains(&week) {
        return None;
    }

    if !(1..=7).contains(&weekday) {
        return None;
    }

    let first_day_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)?;
    let first_weekday_of_month = first_day_of_month.weekday();
    let days_to_first_weekday = (weekday + 7 - first_weekday_of_month.number_from_monday()) % 7;
    let first_target_day =
        first_day_of_month + chrono::Duration::days(days_to_first_weekday as i64);
    let target_date = first_target_day + chrono::Duration::weeks((week - 1) as i64);
    if target_date.month() == date.month() {
        return Some(target_date);
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_week_day() {
        let date = NaiveDate::from_ymd_opt(2024, 9, 11).unwrap();
        assert_eq!(
            week_day(&date, 2, 1),
            Some(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap())
        );

        assert_eq!(
            week_day(&date, 3, 1),
            Some(NaiveDate::from_ymd_opt(2024, 9, 16).unwrap())
        );
        assert_eq!(
            week_day(&date, 3, 5),
            Some(NaiveDate::from_ymd_opt(2024, 9, 20).unwrap())
        );
    }
}
