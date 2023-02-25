use migrations::database::database::{self, Database, DB};
use serde::{Deserialize, Serialize};

use crate::{routes::error::Error};


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Username(String);
impl Username {
    pub fn validate(data: &str) -> Result<Username, Error> {
        // validate that an email is valid
        // let db = database::new().unwrap();
        let q:&str = "SELECT * FROM users WHERE username = '?' LIMIT 1";
        let args: Vec<&str> = vec!["hello"];
        // let q:String = db.prepare(q, &args);
        // let result = db.query(&q);

        let result = DB::fetch(q, args).map_err(|e| Error::Custom(e.to_string()))?;
        if result.result.len() > 0 {
            return Err(Error::Custom("Username already exists".to_string()));
        }
        
        Ok(Username(data.to_string()))
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