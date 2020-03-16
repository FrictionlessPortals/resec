//! A collection of constants used when querying data from the SEC website.

use chrono::prelude::*;
use lazy_static::lazy_static;

/// Examination Material Archive URL
pub(crate) static EXAM_URL: &str = "https://www.examinations.ie/exammaterialarchive/";

lazy_static! {
    /// Examination Paper years offered on the SEC website.
    pub static ref EXAM_PAPER_YEARS: Vec<u32> = {
        let years: Vec<u32> = (1995..Utc::now().year()).map(|x| x as u32).collect();
        years
    };

    /// Examination Paper Marking Schemes years offered on the SEC website.
    pub static ref MARKING_SCHEME_YEARS: Vec<u32> = {
        let years: Vec<u32> = (2001..Utc::now().year()).map(|x| x as u32).collect();
        years
    };
}
