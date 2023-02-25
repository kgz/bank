use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};

use super::VBasicInfo::UserBasicForm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Forms{
    UserBasicForm(UserBasicForm),
}



pub trait BaseForm{
    fn parse(data: &str) -> Result<Forms, HttpResponse>;
    fn validate(data: &Forms) -> Result<Forms, HttpResponse>;

    fn handle(data: &str) -> HttpResponse {
        let data = Self::parse(data);
        if data.is_err() {
            return data.err().unwrap();
        }
        let data = data.unwrap();
        let data = Self::validate(&data);
        if data.is_err() {
            return data.err().unwrap();
        }
        let data = data.unwrap();
        //save to db here
        HttpResponse::Ok().json(data)
    }
}


// impliment a 

