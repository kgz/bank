// use crate::user::roles::roles::get_roles_from_code;

// use super::auth::{self, with_auth, Role};
// use super::error::{self, Error::*};
// use mysql::serde::{Deserialize, Serialize};
// use warp::path::FullPath;
// use std::collections::HashMap;
// use std::convert::Infallible;
// use std::sync::Arc;
// use warp::fs::{file, File};
// use warp::hyper::{Method, Response, StatusCode};
// use warp::{reject, reply, Filter, Rejection, Reply, http}; //
// pub(crate) type Result<T> = std::result::Result<T, error::Error>;
// pub(crate) type WebResult<T> = std::result::Result<T, Rejection>;
// type Users = Arc<HashMap<String, User>>;

// trait FilterClone: Filter + Clone {}
// pub type One<T> = (T,);

// #[derive(Debug, Clone)]
// pub struct User {
//     pub uid: String,
//     pub email: String,
//     pub pw: String,
//     pub role: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct LoginRequest {
//     pub email: String,
//     pub pw: String,
// }
// #[derive(Debug, Deserialize)]

// pub struct GetCode {
//     pub code: String,
// }
// #[derive(Serialize)]
// pub struct LoginResponse {
//     pub token: String,
// }

// // pub fn _get_userFile(uid: String) -> WebResult<impl Reply> {
// //     // warp::any().map(move || users.clone())
// //     // warp::fs::dir("static/" + uid.as_str() + "/img")
// //     // warp::fs::file("static/user/img/1.jpg")
// //     warp::fs::file("static/user/img/1.jpg");
// //     // return "static/user/img/1.jpg";
// //     Ok(format!("Hello Admin {}", uid))
// // }

// // pub fn check_if_path_matches_id(uid: String, path:String) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
// //     filter_fn_one(|route| {
// //         let path = route.param::<String>("path").unwrap();
// //         if path == uid {
// //             Ok(())
// //         } else {
// //             Err(reject::custom(NotAuthorized))
// //         }
// //     })

// // }

// // pub fn user_file(uid: String) -> WebResult<File> {

// // }

// // pub fn check_access(_:String, uid:String, path:FullPath) -> WebResult<impl Reply> {
// //     // if true
// //     // {
// //     //     return Ok(warp::fs::file("static/user/img/1.jpg"));
// //     // }
// //     // return Err(reject::custom(WrongCredentialsError));

// //     // match path.as_str().contains(uid.as_str()) {
// //     //     true => {
// //     //         return Ok(format!("Hello Admin {}", uid));
// //     //     }
// //     //     false => {
// //     //     }
// //     // }
// //     return warp::reject::not_found()

// // }


// pub async fn gen_routes() {
//     let users = Arc::new(init_users());

//     // let t = migrations::run::run_migrations();
//     // let static_files =  warp::path!("api" / "user" / "img")
//     //     .and(warp::fs::dir("static/user"));

//     // redirect /static to statuc {id}



//     let static_files = warp::path!("static" / String / String)
//     .and(with_auth(Role::User))
//     .and(warp::path::full())
//     .and(|_:String, __: String, uid:String, path:FullPath|  {
//         println!("{} {}", uid, path.as_str());
//         // get filename minus extension
//         let file_name = path.as_str().split("/").last().unwrap();
//         let file_name = file_name.split(".").next().unwrap();
//         println!("{} {}", uid, file_name);
//         // return Result<Response<warp::fs::File>, Rejection>

//         if uid == file_name {
//             return Response::builder()
//             .header("content-type", "text/plain")
//             .body(warp::fs::file(format!("static/user/img/{}", path.as_str())))
//             .unwrap()
   

//         } else {
//             return  Response::builder()
//             .status(StatusCode::NOT_FOUND)
//             .body(warp::fs::file(format!("asd")))
//             .unwrap()
            
//         }


//         // if uid == file_name {
//         //     Ok(warp::fs::file(format!("static/user/img/{}", path.as_str())))
//         // } else {
//         //     Err(reject::custom(WrongCredentialsError))
//         // }
//     });


//     // .and(warp::fs::dir("static/user"))
//     // .and(with_auth(Role::Admin))
//     // .and(warp::path::full())
//     // .and_then(|_:String, __: String, ____: warp::fs::File, uid:String, path:FullPath| async move {
//     //     println!("{} {}", uid, path.as_str());
//     //     // get filename minus extension
//     //     let file_name = path.as_str().split("/").last().unwrap();
//     //     let file_name = file_name.split(".").next().unwrap();
//     //     println!("{} {}", uid, file_name);
//     //     if uid == file_name {
//     //         Ok(warp::fs::file(format!("static/user/img/{}", path.as_str())))
//     //     } else {
//     //         Err(reject::custom(WrongCredentialsError))
//     //     }
//     // });
    
//     // .then(warp::fs::dir("static/user"));
 

//     let login_route = warp::path!("api" / "login")
//         .and(with_users(users.clone()))
//         .and(warp::query::<LoginRequest>())
//         .and_then(login_handler);

//     let user_route = warp::path!("api" / "user")
//         .and(with_auth(Role::User))
//         .and_then(user_handler);

//     let admin_route = warp::path!("api" / "admin")
//         .and(with_auth(Role::Admin))
//         .and_then(admin_handler);

//     let get_perm = warp::path!("api" / "get_perm")
//         .and(warp::query::<GetCode>())
//         .map(|body: GetCode| {
//             println!("{:?}", body.code);
//             let r = get_roles_from_code(body.code.as_str());
//             // if err

//             if r.is_ok() {
//                 return reply::with_status(reply::json(&r.unwrap()), StatusCode::OK);
//             }
//             return reply::with_status(reply::json(&r.unwrap_err()), StatusCode::BAD_REQUEST);
//         });

//     let routes = login_route
//         .or(user_route)
//         .or(admin_route)
//         .or(get_perm)
//         .or(static_files)
//         .recover(error::handle_rejection);

//     let cors = warp::cors()
//         .allow_any_origin()
//         .allow_headers(vec![
//             "Access-Control-Allow-Origin",
//             "Origin",
//             "Accept",
//             "X-Requested-With",
//             "Content-Type",
//             "Authorization",
//         ])
//         .allow_methods(&[Method::GET, Method::POST]);

//     warp::serve(routes.with(cors))
//         .run(([127, 0, 0, 1], 3030))
//         .await;
        
// }

// fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
//     warp::any().map(move || users.clone())
// }

// pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
//     println!("{:?}", body);
//     match users
//         .iter()
//         .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
//     {
//         Some((uid, user)) => {
//             let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
//                 .map_err(|e| reject::custom(e))?;
//             Ok(reply::json(&LoginResponse { token }))
//         }
//         None => Err(reject::custom(WrongCredentialsError)),
//     }
// }

// pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
//     Ok(format!("Hello User {}", uid))
// }

// pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
//     Ok(format!("Hello Admin {}", uid))
// }

// fn init_users() -> HashMap<String, User> {
//     let mut map = HashMap::new();
//     map.insert(
//         String::from("1"),
//         User {
//             uid: String::from("1"),
//             email: String::from("user@userland.com"),
//             pw: String::from("1234"),
//             role: String::from("User"),
//         },
//     );
//     map.insert(
//         String::from("2"),
//         User {
//             uid: String::from("2"),
//             email: String::from("admin@adminaty.com"),
//             pw: String::from("4321"),
//             role: String::from("Admin"),
//         },
//     );
//     map
// }
