use std::fmt::Debug;

use actix_web::HttpResponse;
use migrations::database::database::{DB, Database, self};
use serde::{Deserialize, Serialize};
use crate::{validators::{inputs::{username::Username, email::{Email}}, self}, routes::error::{format_serd_error, Error}};

use super::Base::{BaseForm, Forms}; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserBasicForm{
    email: Email, //TODO change later leaving for demo
    username: Username,
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


    fn validate(data: &Forms, id:&String) -> Result<Forms, HttpResponse> {
        if false {
            return Err(HttpResponse::BadRequest().json("Invalid data: validation error"));
        }
          // make data not a reference
          let data = match data {
            Forms::UserBasicForm(data) => data.clone(),
        };

        let q:&str = "SELECT * FROM users WHERE 
            username = ? AND NOT `id` = 1 LIMIT 1";
        let db = database::new().unwrap();
            
        let args: &[&str] = &[&data.username.to_str()];
        let result = db.query(q, Some(args));
        println!("result: {:?}", result);
        if result.results.len() > 0 {
            return Err(HttpResponse::BadRequest().json("Username already exists"));
        }
        //update username and email
        let q:&str = "UPDATE users SET username = ?, email = ? WHERE id = ?";
        let args: &[&str] = &[&data.username.to_str(), &data.email.to_str(), &id];
        DB::fetch(q, args.to_vec()).map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))?;
        Ok(validators::forms::Base::Forms::UserBasicForm(data))
    }


    // pub fn validateIn
}
