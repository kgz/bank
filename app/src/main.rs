// add mysql

extern crate migrations;
use actix_jwt_auth_middleware::AuthResult;
use actix_jwt_auth_middleware::CookieSigner;
use actix_jwt_auth_middleware::FromRequest;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::HttpRequest;
use actix_web::get;
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::header::HeaderMap;
use jwt_compact::alg::Ed25519;
use migrations::database::database::Database;
use migrations::run_migrations;
use migrations::database::database::DB;
use migrations::database::database::Ret;
use migrations::database::database::new;
use jwt_compact::{prelude::*, alg::{Hs256, Hs256Key}};

use actix_web::{guard, web, App, HttpResponse};
use routes::auth::Role;
use routes::auth::authorize;
use routes::auth::create_jwt;
use routes::error::Error;
use serde::Deserialize;
use serde::Serialize;

use crate::routes::auth::jwt_from_header;


mod database {}

mod routes {
    pub mod routes;
    pub mod error;
    pub mod auth;

}
mod api {
    pub mod admin;
    pub mod migrations;
}

mod user {
    pub mod roles;
}

#[allow(dead_code)]
fn create_hello()  {
    let db: DB = new().unwrap();
    let q:&str = "INSERT INTO `test` (`name`) VALUES ('?')";
    let args: Vec<&str> = vec!["hello"];
    let q:String = db.prepare(q, &args);
    let r:Ret = db.query(&q);
    // print r
    // assert_eq r.last != 0
    println!("r: {:?}", r);


}

// #[tokio::main]
// async fn main() {
//     let _db:DB = new().unwrap();
//     run_migrations();
//     routes::routes::gen_routes().await;
//     // let code = user::roles::roles::generate_code();
//     // println!("{}", code);
//     // // get roles
//     // let roles = user::roles::roles::get_roles_from_code(&code);
//     // println!("{:?}", roles);
// }
#[derive(Serialize, Deserialize, Clone, Debug, FromRequest)]
struct User {
    id: u32,
}



async fn index(req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let res = authorize((Role::User, headers)).await;
    // if error 
    if let Err(e) = res {
        return HttpResponse::Unauthorized().body(e.to_string());
    }
    HttpResponse::Ok().body("Hello")
}

async fn login (req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let res = create_jwt("1", &Role::User).await;
    HttpResponse::Ok().body(format!("Bearer {:?}", res))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(testFactory)
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}