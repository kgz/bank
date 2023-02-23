use std::fmt::{self, Display};
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};
use warp::hyper::body::HttpBody;
use std::error;

use crate::routes::error::Error;
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn validate(data: &str) -> Result<Email, Error> {
        // validate that an email is valid
        // let e = MyError::YouAreNotBob("Invalid email".to_string());
        // println!("error: {:?}", e.to_string());
        // Err(Error::FormValidationError("Invalid email".to_owned()))
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

