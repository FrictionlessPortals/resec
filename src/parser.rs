//! A collection of parser functions to allow scraping of
//! data from the website.

use crate::{
    consts::EXAM_URL,
    error::{SecError, SecResult},
    stages::StageBuilder,
};
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name},
};
use std::collections::HashMap;

/// Scrape paper types from generated HTML.
pub async fn parse_types() -> SecResult<HashMap<String, String>> {
    // Fetch the stage two HTML.
    let html = StageBuilder::new().agree_flag(true).query().await?;

    // Parse the HTML into objects.
    let document = Document::from(html.as_str());

    // Find the table containing the paper types.
    let items: Vec<Node> = document
        .find(Attr("name", "MaterialArchive__noTable__sbv__ViewType"))
        .collect();

    // Grab the options.
    match items.get(0) {
        Some(i) => {
            // Create an empty hashmap.
            let mut map = HashMap::new();

            // Loop through the option fields.
            for item in i.find(Name("option")) {
                // Check if the value attribute is empty.
                let value = match item.attr("value") {
                    Some(x) => {
                        if !x.is_empty() {
                            x
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                };

                // Finally, merge the result.
                map.insert(value.into(), item.text());
            }

            // Return the map.
            Ok(map)
        }
        None => Err(SecError::Value("could not get paper type field")),
    }
}

/// Scrape paper years from generated HTML.
pub async fn parse_years(type_id: &str) -> SecResult<Vec<u32>> {
    // Fetch the stage three HTML.
    let html = StageBuilder::new()
        .agree_flag(true)
        .paper_type(type_id)
        .query()
        .await?;

    // Parse the HTML into objects.
    let document = Document::from(html.as_str());

    // Find the table containing the paper types.
    let items: Vec<Node> = document
        .find(Attr("name", "MaterialArchive__noTable__sbv__YearSelect"))
        .collect();

    // Grab the options.
    match items.get(0) {
        Some(i) => {
            // Create an empty vec.
            let mut map = Vec::new();

            // Loop through the option fields.
            for item in i.find(Name("option")) {
                // Check if the value attribute is empty.
                let value: u32 = match item.attr("value") {
                    Some(x) => {
                        if !x.is_empty() {
                            x.parse()?
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                };

                // Finally, merge the result.
                map.push(value);
            }

            // Return the map.
            Ok(map)
        }
        None => Err(SecError::Value("could not get paper year field")),
    }
}

/// Scrape examinations from generated HTML.
pub async fn parse_exams(type_id: &str, year: u32) -> SecResult<HashMap<String, String>> {
    // Fetch the stage two HTML.
    let html = StageBuilder::new()
        .agree_flag(true)
        .paper_type(type_id)
        .year(year)
        .query()
        .await?;

    // Parse the HTML into objects.
    let document = Document::from(html.as_str());

    // Find the table containing the paper types.
    let items: Vec<Node> = document
        .find(Attr(
            "name",
            "MaterialArchive__noTable__sbv__ExaminationSelect",
        ))
        .collect();

    // Grab the options.
    match items.get(0) {
        Some(i) => {
            // Create an empty hashmap.
            let mut map = HashMap::new();

            // Loop through the option fields.
            for item in i.find(Name("option")) {
                // Check if the value attribute is empty.
                let value = match item.attr("value") {
                    Some(x) => {
                        if !x.is_empty() {
                            x
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                };

                // Finally, merge the result.
                map.insert(value.into(), item.text());
            }

            // Return the map.
            Ok(map)
        }
        None => Err(SecError::Value("could not get exam field")),
    }
}

/// Scrape exam subjects from generated HTML.
pub async fn parse_subjects(
    type_id: &str,
    year: u32,
    exam_id: &str,
) -> SecResult<HashMap<u32, String>> {
    // Fetch the stage two HTML.
    let html = StageBuilder::new()
        .agree_flag(true)
        .paper_type(type_id)
        .year(year)
        .examination(exam_id)
        .query()
        .await?;

    // Parse the HTML into objects.
    let document = Document::from(html.as_str());

    // Find the table containing the paper types.
    let items: Vec<Node> = document
        .find(Attr("name", "MaterialArchive__noTable__sbv__SubjectSelect"))
        .collect();

    // Grab the options.
    match items.get(0) {
        Some(i) => {
            // Create an empty hashmap.
            let mut map = HashMap::new();

            // Loop through the option fields.
            for item in i.find(Name("option")) {
                // Check if the value attribute is empty.
                let value = match item.attr("value") {
                    Some(x) => {
                        if !x.is_empty() {
                            x.parse()?
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                };

                // Finally, merge the result.
                map.insert(value, item.text());
            }

            // Return the map.
            Ok(map)
        }
        None => Err(SecError::Value("could not get subject field")),
    }
}

/// Scrape exam papers from the generated HTML.
pub async fn parse_papers(
    type_id: &str,
    year: u32,
    exam_id: &str,
    subject: u32,
) -> SecResult<Vec<(String, String)>> {
    // Fetch the stage two HTML.
    let html = StageBuilder::new()
        .agree_flag(true)
        .paper_type(type_id)
        .year(year)
        .examination(exam_id)
        .subject(subject)
        .query()
        .await?;

    // Parse the HTML into objects.
    let document = Document::from(html.as_str());

    // Generate the inner contents of each document block.
    let contents: Vec<String> = document
        .find(Class("materialbody"))
        .map(|node| filter_node(node))
        .collect();

    // Check if there is material.
    if contents.is_empty() {
        // Return a query failure.
        Err(SecError::NoMaterial)
    } else {
        // Return the parsed material.
        Ok(contents
            .as_slice()
            .chunks(2)
            .map(|chunk| {
                // Split chunk into name and link.
                (
                    chunk
                        .get(0)
                        .expect("could not get material value 0")
                        .to_owned(),
                    chunk
                        .get(1)
                        .expect("could not get material value 1")
                        .to_owned(),
                )
            })
            .collect())
    }
}

/// Filter the material body.
///
/// This function produces either the raw query name
/// for the material or the download link for the material.
fn filter_node(node: Node) -> String {
    // Check if the node contains "Click Here".
    if !node.text().trim().contains("Click Here") {
        node.text().trim().to_string()
    } else {
        // Loop through and filter by hyperlink.
        for x in node.children().filter(|x| x.name() == Some("a")) {
            // Check if it contains a link.
            if x.attr("href").is_some() {
                let x = match x.attr("href") {
                    Some(x) => x,
                    None => return String::new(),
                };

                // If the link contains the EXAM_URL, return the string.
                if x.contains("https://www.examinations.ie") {
                    return x.to_string();
                } else {
                    return format!("{}/{}", EXAM_URL, x.to_string());
                }
            }
        }

        // If all other branches fail.
        String::new()
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::consts::EXAM_PAPER_YEARS;

    #[tokio::test]
    async fn paper_type() -> SecResult<()> {
        // Expected HashMap
        let mut result = HashMap::new();
        result.insert("exampapers".into(), "Exam Papers".into());
        result.insert("markingschemes".into(), "Marking Schemes".into());

        // Parse the paper types.
        let output = parse_types().await?;
        Ok(assert_eq!(output, result))
    }

    #[tokio::test]
    async fn paper_years() -> SecResult<()> {
        // Parse the paper years.
        let output = parse_years("exampapers").await?;
        let output: Vec<u32> = output.into_iter().rev().collect();
        Ok(assert_eq!(output, *EXAM_PAPER_YEARS))
    }

    #[tokio::test]
    async fn examinations() -> SecResult<()> {
        // Expected HashMap
        let mut result = HashMap::new();
        result.insert("lb".into(), "Leaving Certificate Applied".into());
        result.insert("lc".into(), "Leaving Certificate".into());
        result.insert("jc".into(), "Junior Certificate / Cycle".into());

        // Parse the examinations.
        let output = parse_exams("exampapers", 2019).await?;
        Ok(assert_eq!(output, result))
    }

    #[tokio::test]
    async fn subjects() -> SecResult<()> {
        // Expected HashMap
        let mut result = HashMap::new();
        result.insert(14, "Art".into());
        result.insert(2, "English".into());
        result.insert(10, "French".into());
        result.insert(11, "German".into());
        result.insert(1, "Irish".into());

        // Parse the exam subjects.
        let output = parse_subjects("exampapers", 1995, "lc").await?;
        Ok(assert_eq!(output, result))
    }
}
