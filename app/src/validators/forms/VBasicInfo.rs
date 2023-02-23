use std::fmt::Debug;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize, de::Error};
use crate::{validators::{inputs::{username::Username, email::{Email}}, self}, routes::error::format_serd_error};
use itertools::Itertools;
use super::Base::{BaseForm, Forms}; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserBasicForm{
    email: Email, //TODO change later leaving for demo
    username: String,
} 

pub struct UserBasic {}
impl BaseForm for UserBasic {

    fn parse(input: &str) -> Result<Forms, HttpResponse> {
        let data: Result<UserBasicForm, serde_json::Error> = serde_json::from_str(&input);
        if data.is_err() {
            let err = data.unwrap_err().to_string();
            println!("err: {:?}", err);
            return Err(HttpResponse::BadRequest().json(format_serd_error(err)));
        }
        let binding = data.unwrap();
        println!("binding: {:?}", binding);
        Ok(validators::forms::Base::Forms::UserBasicForm(binding))
    }


    fn validate(data: &Forms) -> Result<Forms, HttpResponse> {
        if false {
            return Err(HttpResponse::BadRequest().json("Invalid data: validation error"));
        }

        //    check that email, username, remember_me are set

        if let Forms::UserBasicForm(data) = data {
            // if data.username.is_empty() {
            //     return Err(HttpResponse::BadRequest().json("Invalid data: username is empty"));
            // }
        }

        // make data not a reference
        let data = match data {
            Forms::UserBasicForm(data) => data.clone(),
        };

        

        Ok(validators::forms::Base::Forms::UserBasicForm(data))
    }


    // pub fn validateIn
}
