extern crate migrations;

use actix_files as fs;
use actix_web::HttpResponse;
use actix_web::web;
use actix_web::web::Bytes;
use base64::Engine;
use base64::engine::general_purpose;
use migrations::database::database;
use migrations::database::database::Database;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use actix_web::HttpRequest;
use regex::Regex;
use actix_web::Result;
use std::fs as file;

use crate::routes::error::Error;
use crate::types::user::User;
use crate::validators::Validators::Form;
use crate::validators::forms::VBasicInfo::UserBasic;
use crate::validators::forms::VBasicInfo::UserBasicForm;
// use crate::validators::Validators::Validators;

use super::auth::authorize;

use super::auth::Role;
use actix_files::NamedFile;


pub async fn user_icon(req: HttpRequest) -> Result<fs::NamedFile> {
    let headers = req.headers();
    let res = authorize((Role::User, headers)).await;
    let mut path1;// = PathBuf::from("static/user/404.png");

    if res.is_ok() {
        let user = res.unwrap();
        println!("user: {:?}", user);
        let user_id = user;
        path1 = PathBuf::from(format!("static/user/{}", user_id));
    } else {
        println!("res: {:?}", res);
        return Err(actix_web::error::ErrorNotFound("404"));
    }
    let path2 = PathBuf::from("static/user/404.png");

    if !path1.exists() && !path2.exists() {
        return Err(actix_web::error::ErrorNotFound("404"));
    }

    if !path1.is_file() && !path2.is_file() {
        return Err(actix_web::error::ErrorNotFound("404"));
    }
    println!("path1: {:?}", path1.exists());
    println!("path2: {:?}", path2.exists());
    if !path1.exists() && path2.exists() {
        path1 = path2;
    }

    
    Ok(NamedFile::open(path1)?)
}

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    img: String, //is base 64 string
}

pub async fn set_user_icon(info: web::Json<Info>, req: HttpRequest) -> Result<HttpResponse> {
    // println!("req: {:?}", info);
    // save to file

    let headers = req.headers();
    let res = authorize((Role::User, headers)).await;

    if !res.is_ok() {
       
        println!("res: {:?}", res);
        return Err(actix_web::error::ErrorNotFound("404"));
    }
    let user = res.unwrap();
    println!("user: {:?}", user);
    let user_id = user;

    let mut image_base64_data = info.img.to_owned();

    // get image type
    let image_type = image_base64_data.split(",").next().unwrap();
    let image_type = image_type.split(";").next().unwrap();
    let image_type = image_type.split(":").last().unwrap();
    let image_type = image_type.split("/").last().unwrap();
    println!("image_type: {:?}", image_type);

    let allowed = ["png", "jpg", "jpeg", "gif", "bmp"];

    if !allowed.contains(&image_type) {
        let res = Res {
            status: "error".to_string(),
            message: "image type not allowed".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(res));
    }

    let re = Regex::new(r"^data:image/[a-z]+;base64,").unwrap();
    image_base64_data = re.replace_all(&image_base64_data, "").to_string();
    // let img_data = Engine::decode(&image_base64_data, general_purpose::STANDARD);
    let img_data = general_purpose::STANDARD.decode(&image_base64_data);
    let img_data = img_data.unwrap();
    let path = format!("static/user/{}", user_id);
    file::write(path, img_data).unwrap();
    let res = Res {
        status: "ok".to_string(),
        message: "ok".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_user_detail(req: HttpRequest) -> Result<HttpResponse> {
    let headers = req.headers();
    let user_id = authorize((Role::User, headers)).await;

    if !user_id.is_ok() {
        return Err(actix_web::error::ErrorNotFound("404"));
    }

    let user_id = user_id.unwrap();
    
    let db = database::new().unwrap();
    let q:&str = "SELECT * FROM  `users` WHERE `id` = ?";
    let args: Vec<&str> = vec![&user_id];
    let q:String = db.prepare(q, &args);
    let res = db.query(&q);

    if res.result.len() < 1 {
        let res = Res {
            status: "error".to_string(),
            message: "error".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(res));
    }

    let row = res.result.first().unwrap();
    let user : User = User {
        id: row.get("id").unwrap(),
        username: row.get("username").unwrap(),
        email: row.get("email").unwrap(),
        created_at: row.get("created_at").unwrap(),
        updated_at: row.get("updated_at").unwrap(),
        last_login_attempt: row.get("last_login_attempt").unwrap(),

       
    };

    Ok(HttpResponse::Ok().json(user))
}


trait UserTrait {
    
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Vtype {
    validation_type: String,
    // email: String,
}

pub async fn update_me(bytes: Bytes, req: HttpRequest) -> Result<HttpResponse> {
    // tostring
    let orig_body = String::from_utf8(bytes.to_vec()).unwrap();
    let body: Vtype = serde_json::from_str(&orig_body).unwrap();
    let validate: Result<String, Error>;
    println!("data: {:?}", body.validation_type.as_str());
    match body.validation_type.as_str() {
        "userBasic" => {
            let data: UserBasicForm = serde_json::from_str(&orig_body).unwrap();
            println!("data: {:?}", data);
            validate = UserBasic::validate(&data);
        },
        // "userPassword" => {
        //     let data: UserPasswordForm = serde_json::from_str(&orig_body).unwrap();
        // },


        _ => {
            validate = Err(Error::InvalidData);
        }

    }

    

    println!("data: {:?}", validate);


    // let headers = req.headers();
    // let user_id = authorize((Role::User, headers)).await;

    // if !user_id.is_ok() {
    //     return Err(actix_web::error::ErrorUnauthorized("401"));
    // }
    // let user_id = user_id.unwrap();


    Ok(HttpResponse::Ok().json("asdf"))
}