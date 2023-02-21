use crate::{routes::error::Error, validators::Validators::Input};


pub struct Email {}
impl<'a> Input <'a> for Email {
    fn validate(data: &str) -> Result<String, Error> {
        // validate that an email is valid
        let email = data.to_string();
        let email = email.trim();
        
        if email.len() < 5 {
            return Err(Error::InvalidData);
        }
        if email.len() > 100 {
            return Err(Error::InvalidData);
        }
        if email.contains(" ") {
            return Err(Error::InvalidData);
        }
        if !email.contains("@") {
            return Err(Error::InvalidData);
        }
        if !email.contains(".") {
            return Err(Error::InvalidData);
        }
        if email.contains("..") {
            return Err(Error::InvalidData);
        }
        if email.contains("@.") {
            return Err(Error::InvalidData);
        }
        if email.contains(".@") {
            return Err(Error::InvalidData);
        }
        if email.contains("@@") {
            return Err(Error::InvalidData);
        }
        Ok(data.to_string())
    }
}
