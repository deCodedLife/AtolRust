use std::fs;
use std::string::ToString;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub run_on_logon: bool,
    pub user_email: String,
    pub user_password: String
}

pub const  CONFIG_FILE: &str = "AtolServer.cfg";

impl AppSettings {
    pub fn new() -> Result<AppSettings, Box<dyn std::error::Error>> {
        if fs::metadata(CONFIG_FILE).is_ok() {
            return Self::from_file(CONFIG_FILE);
        }
        Self::to_file(Self::default(), CONFIG_FILE)?;
        Ok(Self::default())
    }
    pub fn default() -> AppSettings {
        AppSettings {
            run_on_logon: false,
            user_email: "test@oxbox.ru".to_string(),
            user_password: "12345".to_string()
        }
    }
    pub fn to_file(self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_content = serde_json::to_string(&self).unwrap();
        Ok(fs::write(filename, file_content)?)
    }
    pub fn from_file(filename: &str) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(filename)?;
        Ok(serde_json::from_str::<AppSettings>(&file_content)?)
    }
}