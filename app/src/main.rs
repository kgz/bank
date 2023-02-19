extern crate migrations;
extern crate argon2;

use actix_files::{self as fs, NamedFile};
use actix_jwt_auth_middleware::FromRequest;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use migrations::database::database::{new, Database, Ret, DB, self as db};
use routes::{auth::login, user::set_user_icon};
use routes::user::{user_icon, get_user_detail};
use serde::{Deserialize, Serialize};

use crate::routes::auth::check_password;

pub struct Env<'a> {
    pub env: &'a str, // one of "dev", "test", "prod"
    pub auto_login_id: &'a str,

}

pub const APP_ENV: Env = Env {
    env: "deav",
    auto_login_id: "1",
};

mod database {}

mod routes {
    pub mod auth;
    pub mod error;
    pub mod user;
}
mod api {
    pub mod admin;
    pub mod migrations;
}

mod user {
    pub mod roles;
}

mod types {
    pub mod user;
}

#[allow(dead_code)]
fn create_hello() {
    let db: DB = new().unwrap();
    let q: &str = "INSERT INTO `test` (`name`) VALUES ('?')";
    let args: Vec<&str> = vec!["hello"];
    let q: String = db.prepare(q, &args);
    let r: Ret = db.query(&q);
    // print r
    // assert_eq r.last != 0
    println!("r: {:?}", r);
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRequest)]
struct User {
    id: u32,
}

async fn index(_: HttpRequest) -> Result<fs::NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn static_media(req: HttpRequest) -> Result<fs::NamedFile> {
    let file = req.match_info().get("file").unwrap();
    let path = format!("static/media/{}", file);
    Ok(NamedFile::open(path)?)
}


pub struct ResUser {
    // id: u32,
    // name: String,
    // email: String,
    // password: String,
}



async fn test(_: HttpRequest) -> Result<String> {
    let pass = "1234567";
    let email = "sa@localhost";
    let conn = db::new().unwrap();


    // alter users  table add column level being int (100) not null default 0

    
    let q:&str = "SELECT * FROM `users` WHERE `email` = '?'";
    let args: Vec<&str> = vec![email];
    let q:String = conn.prepare(q, &args);
    let r = conn.query(&q);
    // get first row
    let row = r.result.first().unwrap();
    println!("row: {:?}", row);
    let res_pass: String = row.get("password").unwrap();
    println!("res_pass: {}", res_pass);

    let r = check_password(pass, &res_pass.to_owned());



    // let hashed = one_way_encrypt(pass);
    // println!("hashed: {}", hashed);
    println!("match: {}", r);
    Ok("test".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    migrations::run_migrations();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/login", web::post().to(login))
            .route("/test", web::get().to(test))
            .service(
                web::resource("/me")
                .name("user_detail_img")
                .route(web::get().to(user_icon))
                .route(web::post().to(set_user_icon))
            )
            .service(
                web::resource("/api/me")
                .name("user_detail")
                .route(web::get().to(get_user_detail))
            )
            .route("/static/media/{file:.*}", web::get().to(static_media))
            //fallback, react will handle the 404
            .route("/{tail:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
