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


mod database {}

mod routes {
    pub mod routes;
    pub mod error;
    pub mod auth;
    pub mod user;

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



// async fn index(req: HttpRequest) -> HttpResponse {
//     let headers = req.headers();
//     let cookies  = req.cookie("token");
//     println!("headers: {:?}", req);
//     let res = authorize((Role::User, headers)).await;
//     // if error 
//     if let Err(e) = res {
//         return HttpResponse::Unauthorized().body(e.to_string());
//     }
//     HttpResponse::Ok().body("Hello")
// }

#[derive(Serialize, Deserialize)]

struct login_res {
    token: String,
}

async fn login (req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let jwt_token = create_jwt("1", &Role::User).await;
    // new dicrt as token: res

    // TODO on unwraop error

    // as serde json
    let res = login_res {
        token: jwt_token.unwrap(),
    };
    
    // HttpResponse::Ok().body("Hello")
    HttpResponse::Ok().json(res)
        
}

async fn index (req: HttpRequest) -> Result<fs::NamedFile>  {
    let headers = req.headers();
    let cookies  = headers.get("cookie");
    println!("headers: {:?}", cookies);
    let res = authorize((Role::User, headers)).await;
    // // if error
    // if let Err(e) = res {
    //     return HttpResponse::Unauthorized().body(e.to_string());
    // }
    Ok(NamedFile::open("static/index.html")?)
}
    // render static/index.html

// #[get("/me")]
// async fn me(req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
//     let f = actix_files::NamedFile::open( "/static/usr/1.jpg").unwrap();
//     // if not f 
//     if  false {
//         return Err(Error::NotFound);
//     }
//     Ok(f)
    
// }



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        // .wrap(
        //     Cors::permissive()
        // )
        // getcookies
        
            
            .route("/", web::get().to(index))
            .route("/api/login", web::get().to(login))
            .route("/me", web::get().to(UserIcon))
            .route("/{_}", web::get().to(index))
            .route("/{_}/{__}", web::get().to(index))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await   
}