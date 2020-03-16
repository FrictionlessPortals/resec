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

/// Main stages enum.
///
/// Allows the user to query different stages easily.
pub enum Stage {
    One(bool),
    Two(String),
    Three(String, i32),
    Four(String, i32, String),
    Five(String, i32, String, String),
}

impl Stage {
    /// Fetch the HTML for a given stage.
    pub async fn fetch_stage(&self) -> SecResult<String> {
        // Prepare the reqwest client.
        let client = Client::new();

        // Generate the stage form body.
        let mut form_layout = HashMap::new();

        // Match the given stage.
        match self {
            Stage::One(a) => {
                // Checkbox Agree Flag
                if *a {
                    // Allow the form to proceed to the second stage.
                    form_layout
                        .insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
                    form_layout
                        .insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());
                }
            }
            Stage::Two(t) => {
                // Checkbox Agree Flag
                form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
                form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());

                // ViewType Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ViewType", t.into());
                form_layout.insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());
            }
            Stage::Three(t, y) => {
                // Checkbox Agree Flag
                form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
                form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());

                // ViewType Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ViewType", t.into());
                form_layout.insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());

                // YearSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__YearSelect", y.to_string());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__YearSelect",
                    "id".to_string(),
                );
            }
            Stage::Four(t, y, e) => {
                // Checkbox Agree Flag
                form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
                form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());

                // ViewType Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ViewType", t.into());
                form_layout.insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());

                // YearSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__YearSelect", y.to_string());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__YearSelect",
                    "id".to_string(),
                );

                // ExaminationSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ExaminationSelect", e.into());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__ExaminationSelect",
                    "id".to_string(),
                );
            }
            Stage::Five(t, y, e, s) => {
                // Checkbox Agree Flag
                form_layout.insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
                form_layout.insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());

                // ViewType Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ViewType", t.into());
                form_layout.insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());

                // YearSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__YearSelect", y.to_string());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__YearSelect",
                    "id".to_string(),
                );

                // ExaminationSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__ExaminationSelect", e.into());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__ExaminationSelect",
                    "id".to_string(),
                );

                // SubjectSelect Flag
                form_layout.insert("MaterialArchive__noTable__sbv__SubjectSelect", s.into());
                form_layout.insert(
                    "MaterialArchive__noTable__sbh__SubjectSelect",
                    "id".to_string(),
                );
            }
        }

        // Post the details using the generated form body.
        let response = client.post(EXAM_URL).form(&form_layout).send().await?;
        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod stages_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn stage_one() -> SecResult<()> {
        // Attempt to get HTML.
        let html = Stage::One(true).fetch_stage().await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_one.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_two() -> SecResult<()> {
        // Attempt to get HTML.
        let html = Stage::Two("exampapers".into()).fetch_stage().await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_two.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_three() -> SecResult<()> {
        // Attempt to get HTML.
        let html = Stage::Three("exampapers".into(), 2019)
            .fetch_stage()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_three.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_four() -> SecResult<()> {
        // Attempt to get HTML.
        let html = Stage::Four("exampapers".into(), 2019, "lc".into())
            .fetch_stage()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_four.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_five() -> SecResult<()> {
        // Attempt to get HTML.
        let html = Stage::Five("exampapers".into(), 2019, "lc".into(), 1.to_string())
            .fetch_stage()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_five.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }
}
