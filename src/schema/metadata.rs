//! The examination paper query metadata.

use std::str::FromStr;
use strum_macros::{Display, EnumIter, EnumProperty, EnumString};

/// The different document types offered on the SEC website.
/// Each type contains its name and ID that can be used to generate a query.
#[derive(EnumProperty, EnumIter, Debug, Clone, PartialEq)]
pub enum Type {
    #[strum(props(name = "Exam_Papers", id = "exampapers"))]
    ExamPaper,
    #[strum(props(name = "Marking_Schemes", id = "markingschemes"))]
    MarkingScheme,
}

/// The different examinations offered on the SEC website.
/// Each examination contains its name and ID that can be used to generate a query.
#[derive(EnumProperty, EnumIter, Debug, Clone, PartialEq)]
pub enum Examination {
    #[strum(props(name = "Leaving_Certificate_Applied", id = "lb"))]
    LeavingCertificateApplied,
    #[strum(props(name = "Leaving_Certificate", id = "lc"))]
    LeavingCertificate,
    #[strum(props(name = "Junior_Certificate/Cycle", id = "jc"))]
    JuniorCertificate,
}

/// The different examination languages offered on the SEC website.
#[derive(EnumString, EnumIter, Display, Debug, Clone, PartialEq)]
pub enum Language {
    #[strum(serialize = "IV")]
    Irish,
    #[strum(serialize = "EV")]
    English,
    NoLanguage,
}

/// Implementation for converting a string name parsed from a query into a language.
impl From<String> for Language {
    fn from(raw_name: String) -> Self {
        let split_name: Vec<&str> = raw_name.split('(').collect();
        match Language::from_str(split_name[1].trim_end_matches(')')) {
            Ok(x) => x,
            Err(_) => Language::NoLanguage,
        }
    }
}

/// The different examination levels offered on the SEC website.
#[derive(EnumString, EnumIter, Display, Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Level {
    #[strum(serialize = "Higher Level")]
    HigherLevel,
    #[strum(serialize = "Ordinary Level")]
    OrdinaryLevel,
    #[strum(serialize = "Foundation Level")]
    FoundationLevel,
    #[strum(serialize = "Common Level")]
    CommonLevel,
    NoLevel,
}

/// Implementation for converting a string name parsed from a query into a level.
impl From<String> for Level {
    fn from(raw_name: String) -> Self {
        let split_name: Vec<&str> = raw_name.split('(').collect();
        let level: Vec<&str> = split_name[0]
            .split('/')
            .map(|x| x.trim_matches(' '))
            .collect();

        match level.get(1) {
            Some(x) => match Level::from_str(x) {
                Ok(x) => x,
                Err(_) => Level::NoLevel,
            },
            None => Level::NoLevel,
        }
    }
}

#[cfg(test)]
mod metadata_tests {
    use super::*;
    use strum::EnumProperty;

    #[test]
    fn parse_type() {
        let paper_type = Type::ExamPaper;
        assert_eq!(
            String::from("Exam_Papers"),
            paper_type.get_str("name").unwrap()
        );
        assert_eq!("exampapers", paper_type.get_str("id").unwrap());
    }

    #[test]
    fn parse_language() {
        let language = Language::Irish;
        assert_eq!(Language::from_str("IV").unwrap(), language);
        assert_eq!("IV", language.to_string());
    }

    #[test]
    fn parse_level() {
        let level = Level::OrdinaryLevel;
        assert_eq!(Level::from_str("Ordinary Level").unwrap(), level);
        assert_eq!("Ordinary Level", level.to_string());
    }

    #[test]
    fn parse_examination() {
        let examination = Examination::LeavingCertificate;
        assert_eq!(
            String::from("Leaving_Certificate"),
            examination.get_str("name").unwrap()
        );
        assert_eq!("lc", examination.get_str("id").unwrap());
    }
}
