extern crate argon2;
extern crate migrations;
use std::thread;

use actix_cors::Cors;

use actix_files::{self as fs, NamedFile};
use actix_jwt_auth_middleware::FromRequest;
use actix_web::{http, web, App, HttpRequest, HttpServer, Result};
use chrono::Utc;
use migrations::database::database::{self as db, new, Database, Ret, DB};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use routes::user::{get_user_detail, update_me, user_icon};
use routes::{auth::login, user::set_user_icon};
use serde::{Deserialize, Serialize};

use crate::cronjobs::api::{start_cron, stop_cron};
use crate::cronjobs::CronManager;
use crate::routes::auth::check_password;

#[derive(PartialEq)]
pub enum Environments {
    DEV,
    TEST,
    PROD,
}

pub struct Env<'a> {
    pub env: Environments,
    pub auto_login_id: &'a str,
}

pub const APP_ENV: Env = Env {
    env: Environments::DEV,
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

pub mod validators {
    pub mod forms {
        automod::dir!(pub "src/validators/forms");
    }
    pub mod inputs {
        automod::dir!(pub "src/validators/inputs");
    }
}

pub mod cronjobs {
    pub mod CronManager;
    pub mod cronInterface;
    pub mod crons {
        automod::dir!(pub "src/cronjobs/crons");
    }

    pub mod api;
}

#[allow(dead_code)]
fn create_hello() {
    let db: DB = new().unwrap();
    let q: &str = "INSERT INTO `test` (`name`) VALUES ('?')";
    let args: Vec<&str> = vec!["hello"];
    let q: String = db.prepare(q, &args);
    let r: Ret = db.query(&q, None);
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

async fn test(_: HttpRequest) -> Result<String> {
    let pass = "1234567";
    let email = "sa@localhost";
    let conn = db::new().unwrap();
    let q: &str = "SELECT * FROM `users` WHERE `email` = '?'";
    let args: Vec<&str> = vec![email];
    let q: String = conn.prepare(q, &args);
    let r = conn.query(&q, None);
    // get first row
    let row = r.results.first().unwrap();
    println!("row: {:?}", row);
    let res_pass: String = row.get("password").unwrap();
    println!("res_pass: {}", res_pass);

    let r = check_password(pass, &res_pass.to_owned());

    println!("match: {}", r);
    Ok("test".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    migrations::run_migrations();
    // print cwd()
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(".pem/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(".pem/cert.pem").unwrap();

    let manager = CronManager::CronManager::new();
    thread::spawn(move || {
        manager.run();
    });

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://localhost")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/api/login", web::post().to(login))
            .route("/test", web::get().to(test))
            .service(
                web::resource("/me")
                    .name("user_detail_img")
                    .route(web::get().to(user_icon))
                    .route(web::post().to(set_user_icon)),
            )
            .service(
                web::resource("/api/me")
                    .name("user_detail")
                    .route(web::get().to(get_user_detail)),
            )
            .service(
                web::resource("/api/me/update")
                    .name("update_me")
                    .route(web::post().to(update_me)),
            )
            // .service(
            //     web::resource("/api/cron/start")
            //     .name("cronstrat")
            //     .app_data(cron)
            //     .route(web::get().to(start_cron))
            // )
            // .route("/api/cron/start", web::get().to(|req: HttpRequest|start_cron(req, &manager)))
            // .route("/api/cron/stop", web::get().to(|req: HttpRequest|stop_cron(req, &manager)))
            .route("/static/media/{file:.*}", web::get().to(static_media))
            //fallback, react will handle the 404
            .route("/{tail:.*}", web::get().to(index))
    })
    .bind_openssl("0.0.0.0:443", builder)?;

    // print server url
    println!("Server running at https://localhost:443/");
    server.run().await
}
