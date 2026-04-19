use std::fs;
use std::io::Write;

use anyhow::Result;
use dirs::data_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let filename = data_dir().unwrap().join("RejectionRoulette/settings.toml");

        if filename.exists() {
            let tmp = fs::read_to_string(&filename).unwrap();

            toml::from_str(&tmp).unwrap()
        } else {
            let settings = Self::default();
            settings.save().unwrap();
            settings
        }
    }

    pub fn save(&self) -> Result<()> {
        let filename = data_dir().unwrap().join("RejectionRoulette/settings.toml");
        let data = toml::to_string_pretty(&self)?;
        let mut file = fs::File::create(&filename)?;

        file.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn get_eval_stmt(&self) -> Result<String> {
        let stmt = format!(
            "document.documentElement.setAttribute('data-theme', '{}')",
            self.theme
        );

        Ok(stmt)
    }
}
