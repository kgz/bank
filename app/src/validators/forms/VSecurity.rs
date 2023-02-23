use serde::{Deserialize, Serialize};
// use std::result::Result;
use crate::{routes::{error::Error, self}};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSecurityForm {
    email: String,
    username: String,
} 
impl  UserSecurityForm {}

pub struct SecurityForm {}
impl SecurityForm {

    pub fn validate(data: SecurityForm) -> Result<String, Error> {
        if false {
            return Err(Error::InvalidData);
        }
        Ok("ok".to_string())
    }

    pub fn parse(input: &str) -> Result<UserSecurityForm, Error> {
        todo!()
    }
}
