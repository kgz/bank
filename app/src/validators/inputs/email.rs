use serde::{Deserialize, Serialize};

use crate::routes::error::Error;
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn validate(data: &str) -> Result<Email, Error> {
        let regex = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if !regex.is_match(data)
            || data.len() > 254
            || data.len() < 5
            || data.contains("..")
            || data.contains("._")
            || data.contains("_.")
            || data.contains("__")
            || data.contains("._.")
            || data.contains("._-")
            || data.contains("-._")
            || data.contains("-.-")
            || data.contains("-_-")
            || data.is_empty()
        {
            return Err(Error::Custom("Invalid email".to_string()));
        }

        


        Ok(Email(data.to_string()))
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let r = Self::validate(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(r)
    }
}
