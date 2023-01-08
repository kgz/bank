use super::auth::{with_auth, Role, self};
use super::error::{Error::*, self};
use mysql::serde::{Serialize, Deserialize};
use warp::http::HeaderValue;
use warp::hyper::{HeaderMap, Method, Response, StatusCode};
use warp::reply::with_header;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{reject, reply, Filter, Rejection, Reply};


pub(crate) type Result<T> = std::result::Result<T, error::Error>;
pub(crate) type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn gen_routes() {
    // let t = migrations::run::run_migrations();

        let users = Arc::new(init_users());
        
        let login_route = warp::path!("api" / "login")
            // .and(with_users(users.clone()))
            // .and(warp::query::<LoginRequest>())
            // .and_then(login_handler);
            .map(|| "Hello, world!");

        let user_route = warp::path!("api" / "user")
            .and(with_auth(Role::User))
            .and_then(user_handler);
        let admin_route = warp::path!("api" / "admin")
            .and(with_auth(Role::Admin))
            .and_then(admin_handler);

        //     login_route
            // .and(
            //     // .or(user_route)
            //     // .or(admin_route)
            // );
            // .recover(error::handle_rejection);
        let cors = warp::cors()
            .allow_any_origin();
            // .allow_methods(vec!["GET", "POST", "DELETE"]);
            // .allow_origin("http://localhost")
            // // add add Access-Control-Allow-Origin "*"
            
            // .allow_methods(vec!["POST", "GET"]);
        let mut headers = HeaderMap::new();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("POST, GET, OPTIONS, PUT, DELETE"));
        headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Access-Control-Allow-Headers, Access-Control-Request-Method, Access-Control-Request-Headers, Origin, Accept, X-Requested-With, Content-Type"));
        headers.insert("Access-Control-Allow-Credentials", HeaderValue::from_static("true"));

        // let response_headers = warp::reply::with::headers(headers);


        let routes = login_route.with(warp::reply::with::headers(headers));
      
        let cors = warp::cors().allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Headers", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Origin", "Accept", "X-Requested-With", "Content-Type"])
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS, Method::HEAD]);

        

        warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;
       // allows cors for port 80 on same domain
    // .with(warp::cors().allow_any_origin())
//     warp::serve(routes.with(warp::cors().allow_any_origin()))
//         .run(([127, 0, 0, 1], 3030))
//         .await;
}


fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    println!("{:?}", body);
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );
    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@adminaty.com"),
            pw: String::from("4321"),
            role: String::from("Admin"),
        },
    );
    map
}