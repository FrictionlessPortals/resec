//! The different HTML stages for the examination archive.
//!
//! In total there are 6 stages:
//!
//! 1. Terms and Conditions
//! 2. Choose Type
//! 3. Choose Year
//! 4. Choose Examination
//! 5. Choose Subject
//! 6. Paper Choices
//!
//! These stages follow each other and require the
//! previous one to be active in order to work.

use crate::{consts::EXAM_URL, error::SecResult};
use reqwest::Client;
use std::collections::HashMap;

/// Fetch the HTML for stage one.
///
/// If the user agrees to the terms it allows the user to probe the second stage.
pub async fn fetch_stage_one(checked: bool) -> SecResult<String> {
    // Prepare the reqwest client.
    let client = Client::new();

    // Generate the stage one form body.
    let mut form_layout = HashMap::new();

    // Checkbox Agree Flag
    if checked {
        // Allow the form to proceed to the second stage.
        form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
        form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());
    }

    // Post the details using the generated form body.
    let response = client.post(EXAM_URL).form(&form_layout).send().await?;
    Ok(response.text().await?)
}

/// Fetch the HTML for stage two.
///
/// If the user enters a correct type ID it allows the user
/// to probe the third stage.
pub async fn fetch_stage_two(type_id: String) -> SecResult<String> {
    // Prepare the reqwest client.
    let client = Client::new();

    // Generate the stage one form body.
    let mut form_layout = HashMap::new();

    // Checkbox Agree Flag
    form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
    form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());

    // ViewType Flag
    form_layout.insert("MaterialArchive__noTable__sbv__ViewType", type_id);
    form_layout.insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());

    // Post the details using the generated form body.
    let response = client.post(EXAM_URL).form(&form_layout).send().await?;
    Ok(response.text().await?)
}

#[cfg(test)]
mod stages_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn stage_one() -> SecResult<()> {
        // Attempt to get HTML.
        let html = fetch_stage_one(true).await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_one.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_two() -> SecResult<()> {
        // Attempt to get HTML.
        let html = fetch_stage_two("exampapers".into()).await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_two.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }
}
