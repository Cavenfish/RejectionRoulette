use regex::Regex;

use crate::database::{Application, Interview};

#[derive(Default, Debug)]
pub struct FilterCriteria {
    pub company: Option<String>,
    pub role: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
    pub resume: Option<String>,
    pub interview_type: Option<String>,
}

impl FilterCriteria {
    pub fn parse(input: &str) -> Self {
        let mut criteria = Self::default();

        // Regex breakdown:
        // Group 0: Full match
        //
        // Group 1: (\w+)
        //      - Match the word before a colon (e.g. role or company)
        //
        // :\s? matches colon and optional space
        //
        // Groups 2-4 are one conditional (?:"([^"]*)"|'([^']*)'|(\S+))
        //      - They all match the value (e.g. Google)
        //
        // Group 2: "([^"]*)"
        //      - Anything inside double quotes
        //
        // Group 2: '([^']*)'
        //      - Anything inside single quotes
        //
        // Group 4: (\S+)
        //      - All non-whitespace characters
        let token_re = Regex::new(r#"(\w+):\s?(?:"([^"]*)"|'([^']*)'|(\S+))"#).unwrap();

        for cap in token_re.captures_iter(input) {
            let key = cap[1].to_lowercase();

            let value = if cap.get(2).is_some() {
                cap[2].to_lowercase()
            } else if cap.get(3).is_some() {
                cap[3].to_lowercase()
            } else {
                cap[4].to_lowercase()
            };

            match key.as_str() {
                "company" => criteria.company = Some(value),
                "role" => criteria.role = Some(value),
                "location" => criteria.location = Some(value),
                "status" => criteria.status = Some(value),
                "resume" => criteria.resume = Some(value),
                "type" => criteria.interview_type = Some(value),
                _ => continue,
            }
        }

        criteria
    }

    pub fn filter_applications<'a>(
        &self,
        applications: &'a Vec<Application>,
    ) -> impl Iterator<Item = &'a Application> {
        applications
            .iter()
            .filter(move |app| {
                let company = self
                    .company
                    .as_ref()
                    .map_or(true, |c| app.company.to_lowercase().contains(c));

                let role = self
                    .role
                    .as_ref()
                    .map_or(true, |c| app.role.to_lowercase().contains(c));

                let location = self
                    .location
                    .as_ref()
                    .map_or(true, |c| app.location.to_lowercase().contains(c));

                let status = self
                    .status
                    .as_ref()
                    .map_or(true, |c| app.status.to_lowercase().contains(c));

                let resume = self.resume.as_ref().map_or(true, |c| {
                    app.resume
                        .clone()
                        .unwrap_or("--".to_string())
                        .to_lowercase()
                        .contains(c)
                });

                company && role && location && status && resume
            })
            .rev()
    }

    pub fn filter_interviews<'a>(
        &self,
        interviews: &'a Vec<Interview>,
    ) -> impl Iterator<Item = &'a Interview> {
        interviews
            .iter()
            .filter(move |app| {
                let company = self
                    .company
                    .as_ref()
                    .map_or(true, |c| app.company.to_lowercase().contains(c));

                let role = self
                    .role
                    .as_ref()
                    .map_or(true, |c| app.role.to_lowercase().contains(c));

                let interview_type = self
                    .interview_type
                    .as_ref()
                    .map_or(true, |c| app.interview_type.to_lowercase().contains(c));

                company && role && interview_type
            })
            .rev()
    }
}
