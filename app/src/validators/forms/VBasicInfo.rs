use serde::{Deserialize, Serialize};
// use std::result::Result;
use crate::{routes::error::Error, validators::{Validators::{Inputs, Form}, inputs::email::Email}};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserBasicForm {
    email: String,
    username: String,
} 
impl Inputs<'_> for UserBasicForm {}


pub struct UserBasic {}
impl<'a> Form<'a> for UserBasic {
    type Inputs = UserBasicForm;

    fn validate(data: &dyn Inputs) -> Result<String, Error> {
        if false {
            return Err(Error::InvalidData);
        }
        Ok("ok".to_string())
    }
}
