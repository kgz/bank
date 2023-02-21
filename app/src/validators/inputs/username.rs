use crate::{routes::error::Error, validators::Validators::Input};


pub struct Username {}
impl<'a> Input <'a> for Username {
    fn validate(data: &str) -> Result<String, Error> {
        // validate that an email is valid
       
        
        Ok(data.to_string())
    }
}
