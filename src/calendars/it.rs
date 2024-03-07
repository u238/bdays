
use chrono::{Datelike, Weekday, NaiveDate, Duration};
use crate::HolidayCalendar;

/// Italian holidays
pub struct ITSettlement;

fn find_first_monday(yy: i32, mm: u32, dd: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(yy, mm, dd).unwrap()
        .iter_days()
        .find(|d| d.weekday() == Weekday::Mon )
        .unwrap()
}

impl<T: Datelike + Copy + PartialOrd> HolidayCalendar<T> for ITSettlement {

    fn is_holiday(&self, date: T) -> bool {
        let (yy, mm, dd) = (date.year(), date.month(), date.day());
        let dt_naive = NaiveDate::from_ymd_opt(yy, mm, dd).unwrap();

        if
            // Capodanno (1 gennaio)
            NaiveDate::from_ymd_opt(yy, 1, 1).unwrap() == dt_naive
            ||
            // Epifania (6 gennaio)
            NaiveDate::from_ymd_opt(yy, 1, 6).unwrap() == dt_naive
            ||
            // Pasquetta (Lunedi dopo pasqua) FIXME: very approximate
            find_first_monday(yy, 4, 1) == dt_naive
            ||
            // Festa della liberazione (25 aprile)
            NaiveDate::from_ymd_opt(yy, 4, 25).unwrap() == dt_naive
            ||
            // Festa dei lavoratori (1 maggio);
            NaiveDate::from_ymd_opt(yy, 5, 1).unwrap() == dt_naive
            ||
            // Festa della Repubblica (2 giugno);
                NaiveDate::from_ymd_opt(yy, 6, 2).unwrap() == dt_naive
            ||
            // Assunzione di Maria Vergine o Ferragosto (15 agosto);
                NaiveDate::from_ymd_opt(yy, 8, 15).unwrap() == dt_naive
            ||
            // Tutti i Santi (1º novembre);
                NaiveDate::from_ymd_opt(yy, 11, 1).unwrap() == dt_naive
            ||
            // Immacolata Concezione (8 dicembre);
                NaiveDate::from_ymd_opt(yy, 12, 8).unwrap() == dt_naive
            ||
            // Natale (25 dicembre);
                NaiveDate::from_ymd_opt(yy, 12, 25).unwrap() == dt_naive
            ||
            // Santo Stefano (26 dicembre).
                NaiveDate::from_ymd_opt(yy, 12, 25).unwrap() == dt_naive
         {
            return true
        }
        false
    }
}
