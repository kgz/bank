use migrations::database::database::{self, Database, DB};
use serde::{Deserialize, Serialize};

use crate::{routes::error::Error};


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Username(String);
impl Username {
    pub fn validate(data: &str) -> Result<Username, Error> {
        let regex = regex::Regex::new(r"[a-zA-Z0-9_-]").unwrap();
        if !regex.is_match(data) {
            return Err(Error::Custom("Username is invalid, must be a-z, A-Z, 0-9, _ and -".to_string()));
        }
        if data.len() < 3 || data.len() > 16 {
            return Err(Error::Custom("Username must be between 3 and 16 characters".to_string()));
        }
        Ok(Username(data.to_string()))
    }

    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }
}


impl<'de> Deserialize<'de> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let r = Self::validate(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(r)
    }
}