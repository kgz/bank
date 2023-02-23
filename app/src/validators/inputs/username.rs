use crate::{routes::error::Error};


pub struct Username {}
impl Username {
    pub fn validate(data: &str) -> Result<String, Error> {
        // validate that an email is valid
       
        
        Ok(data.to_string())
    }
}
