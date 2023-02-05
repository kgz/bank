extern crate migrations;

use actix_files as fs;
use actix_web::HttpResponse;
use actix_web::web;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use actix_web::HttpRequest;
use regex::Regex;
use actix_web::Result;
use std::fs as file;
use super::auth::authorize;

use super::auth::Role;
use actix_files::NamedFile;

pub async fn user_icon(req: HttpRequest) -> Result<fs::NamedFile> {
    let headers = req.headers();
    let res = authorize((Role::User, headers)).await;
    let mut path1 = PathBuf::from("static/user/404.png");

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
struct res {
    status: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    img: String, //is base 64 string
}

pub async fn set_user_icon(info: web::Json<Info>) -> Result<HttpResponse> {
    println!("req: {:?}", info);
    // save to file

    let mut image_base64_data = info.img.to_owned();

    // get image type
    let image_type = image_base64_data.split(",").next().unwrap();
    let image_type = image_type.split(";").next().unwrap();
    let image_type = image_type.split(":").last().unwrap();
    let image_type = image_type.split("/").last().unwrap();
    println!("image_type: {:?}", image_type);

    let allowed = ["png", "jpg", "jpeg", "gif", "bmp"];

    if !allowed.contains(&image_type) {
        let res = res {
            status: "error".to_string(),
            message: "image type not allowed".to_string(),
        };
        return Ok(HttpResponse::BadRequest().json(res));
    }

    let re = Regex::new(r"^data:image/[a-z]+;base64,").unwrap();
    image_base64_data = re.replace_all(&image_base64_data, "").to_string();
    let img_data = base64::decode(&image_base64_data);
    let img_data = img_data.unwrap();
    let path = format!("static/user/1");
    file::write(path, img_data).unwrap();
    let res = res {
        status: "ok".to_string(),
        message: "ok".to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}
