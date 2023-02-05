// add mysql

extern crate migrations;
use std::path::PathBuf;
use actix_cors::Cors;
use actix_files as fs;


use actix_jwt_auth_middleware::FromRequest;
use actix_web::HttpServer;
use actix_web::HttpRequest;


use migrations::database::database::Database;
use migrations::run_migrations;
use migrations::database::database::DB;
use migrations::database::database::Ret;
use migrations::database::database::new;
use jwt_compact::{prelude::*, alg::{Hs256, Hs256Key}};
use actix_web::{Result};

use actix_web::{guard, web, App, HttpResponse};
use routes::auth::Role;
use routes::auth::authorize;
use routes::auth::create_jwt;
use routes::user::UserIcon;
use serde::Deserialize;
use serde::Serialize;
use actix_files::NamedFile;
use serde_json::json;

use crate::routes::auth::jwt_from_header;


pub async fn UserIcon(req: HttpRequest) -> Result<fs::NamedFile> {

    let headers = req.headers();
    println!("headers: {:?}", headers);
    let res = authorize((Role::User, headers)).await;
    let mut path1 = PathBuf::from("static/user/404.png");

    if(res.is_ok()) {
        let user = res.unwrap();
        println!("user: {:?}", user);
        let user_id = user;
        path1 = PathBuf::from(format!("static/user/{}.jpg", user_id));
    }else{
        println!("res: {:?}", res);
        // return Err(actix_web::error::ErrorNetworkAuthenticationRequired(res.unwrap_err()));
    }
    
    // println!("res: {:?}", res);
    
    //list all files in ./static
    let path2 = PathBuf::from("static/user/404.png");

    if(!path1.exists() && !path2.exists()) {
        return Err(actix_web::error::ErrorNotFound("404"));
    }

    if(!path1.is_file() && !path2.is_file()) {
        return Err(actix_web::error::ErrorNotFound("404"));
    }
    println!("path1: {:?}", path1.exists());
    println!("path2: {:?}", path2.exists());
    if !path1.exists() && path2.exists() {
        path1 = path2;
    }    

    Ok(NamedFile::open(path1)?)
}