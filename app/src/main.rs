// add mysql

extern crate migrations;
use migrations::database::database::Database;
use migrations::run_migrations;
use migrations::database::database::DB;
use migrations::database::database::Ret;
use migrations::database::database::new;

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

#[tokio::main]
async fn main() {
    let _db:DB = new().unwrap();
    run_migrations();
    routes::routes::gen_routes().await;
    // let code = user::roles::roles::generate_code();
    // println!("{}", code);
    // // get roles
    // let roles = user::roles::roles::get_roles_from_code(&code);
    // println!("{:?}", roles);
}
