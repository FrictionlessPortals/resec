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

/// Main stage builder.
pub struct StageBuilder {
    query_form: HashMap<&'static str, String>,
}

impl StageBuilder {
    /// Create a new stage builder.
    pub fn new() -> Self {
        Self {
            query_form: HashMap::new(),
        }
    }

    /// Set terms and conditions checkbox.
    pub fn agree_flag(mut self, flag: bool) -> Self {
        // Check the given flag.
        if flag {
            // Agree to the terms and conditions.
            self.query_form
                .insert("MaterialArchive__noTable__cbv__AgreeCheck", "Y".to_string());
            self.query_form
                .insert("MaterialArchive__noTable__cbh__AgreeCheck", "N".to_string());
        }

        // Return the changed object.
        self
    }

    /// Set paper type.
    pub fn paper_type(mut self, id: &str) -> Self {
        // ViewType Flag
        self.query_form
            .insert("MaterialArchive__noTable__sbv__ViewType", id.into());
        self.query_form
            .insert("MaterialArchive__noTable__sbh__ViewType", "id".to_string());

        // Return the changed object.
        self
    }

    // Set year.
    pub fn year(mut self, year: u32) -> Self {
        // YearSelect Flag
        self.query_form.insert(
            "MaterialArchive__noTable__sbv__YearSelect",
            year.to_string(),
        );
        self.query_form.insert(
            "MaterialArchive__noTable__sbh__YearSelect",
            "id".to_string(),
        );

        // Return the changed object.
        self
    }

    // Set examination.
    pub fn examination(mut self, id: &str) -> Self {
        // ExaminationSelect Flag
        self.query_form.insert(
            "MaterialArchive__noTable__sbv__ExaminationSelect",
            id.into(),
        );
        self.query_form.insert(
            "MaterialArchive__noTable__sbh__ExaminationSelect",
            "id".to_string(),
        );

        // Return the changed object.
        self
    }

    // Set subject.
    pub fn subject(mut self, id: u32) -> Self {
        // SubjectSelect Flag
        self.query_form.insert(
            "MaterialArchive__noTable__sbv__SubjectSelect",
            id.to_string(),
        );
        self.query_form.insert(
            "MaterialArchive__noTable__sbh__SubjectSelect",
            "id".to_string(),
        );

        // Return the changed object.
        self
    }

    // Finish building the stage and query using built object.
    pub async fn query(&self) -> SecResult<String> {
        // Prepare the reqwest client.
        let client = Client::new();

        // Post the details using the generated form body.
        let response = client.post(EXAM_URL).form(&self.query_form).send().await?;
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
        let html = StageBuilder::new().agree_flag(false).query().await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_one.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_two() -> SecResult<()> {
        // Attempt to get HTML.
        let html = StageBuilder::new()
            .agree_flag(true)
            .paper_type("exampapers")
            .query()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_two.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_three() -> SecResult<()> {
        // Attempt to get HTML.
        let html = StageBuilder::new()
            .agree_flag(true)
            .paper_type("exampapers")
            .year(2019)
            .query()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_three.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_four() -> SecResult<()> {
        // Attempt to get HTML.
        let html = StageBuilder::new()
            .agree_flag(true)
            .paper_type("exampapers")
            .year(2019)
            .examination("lc")
            .query()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_four.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    #[tokio::test]
    async fn stage_five() -> SecResult<()> {
        // Attempt to get HTML.
        let html = StageBuilder::new()
            .agree_flag(true)
            .paper_type("exampapers")
            .year(2019)
            .examination("lc")
            .subject(1)
            .query()
            .await?;

        // Open up a file and write the html.
        let mut file = File::create("stage_five.html")?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }
}
