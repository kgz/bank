use crate::{routes::error::Error, APP_ENV};
use actix_web::{http::header::{HeaderMap, AUTHORIZATION}, HttpResponse, web};
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{fmt};
use argon2::{self, Config};
use rand::{distributions::Alphanumeric, Rng, thread_rng};
use migrations::database::database::{Database, self as db};

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

#[derive(Clone, PartialEq, Debug)]
pub enum Role {
    User,
    // Admin,
}



// impl Role {
//     pub fn from_str(role: &str) -> Role {
//         match role {
//             "Admin" => Role::Admin,
//             _ => Role::User,
//         }
//     }
// }

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            // Role::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub async fn create_jwt(uid: &str, role: &Role) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

pub async fn authorize((role, headers): (Role, &HeaderMap)) -> Result<String, Error> {
    if APP_ENV.env == "dev" {
        return Ok(APP_ENV.auto_login_id.to_string());
    }

    let jwt = jwt_from_header(&headers)?;
    println!("{:?}", jwt);
    let token_data = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;
    let claims = token_data.claims;
    if claims.role != role.to_string() {
        return Err(Error::JWTTokenCreationError);
    }
    Ok(claims.sub)

}

pub fn jwt_from_header(headers: &HeaderMap) -> Result<String, Error> {

    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    match !auth_header.starts_with(BEARER) {
        true => return Err(Error::InvalidAuthHeaderError),
        false => (),
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}



#[derive(Serialize, Deserialize)]
pub struct LoginRes {
    token: String,
}

#[derive(Deserialize)]
pub struct LoginReqs {
    email: String,
    password: String,
}

pub async fn login(params: web::Json<LoginReqs>) -> HttpResponse {

    let email = params.email.to_string();
    let password = params.password.to_string();

    let validated: bool = validate_password(email, password).await;
    if validated == false {
        return HttpResponse::Unauthorized().finish();
    }
    let jwt_token = create_jwt("1", &Role::User).await;
    let res = LoginRes {
        token: jwt_token.unwrap(),
    };

    HttpResponse::Ok().json(res)
}


pub fn _one_way_encrypt(password: &str) -> String {
    let password = password.as_bytes();
    let salt: String  = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(|x| x as char)
    .collect();

    // convert to u8
    let salt = salt.as_bytes();

    
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    hash
}


pub fn check_password( password: &str, hash: &str) -> bool {
    let password = password.as_bytes();
    // has as u8
    argon2::verify_encoded(hash, password).unwrap()

}

pub async fn get_user_pass(email: String) -> String {
    let conn = db::new().unwrap();
    let q:&str = "SELECT password FROM `users` WHERE `email` = '?'";
    let args: Vec<&str> = vec![&email];
    let q:String = conn.prepare(q, &args);
    let r = conn.query(&q);
    // get first row
    if r.result.len() == 0 {
        return "".to_string();
    }
    // error on no row
    let row = r.result.first().unwrap();
    let res_pass: String = row.get("password").unwrap();
    res_pass
}

pub async fn _get_user_id_from_email(email:String) -> String{
    let conn = db::new().unwrap();
    let q:&str = "SELECT id FROM `users` WHERE `email` = '?'";
    let args: Vec<&str> = vec![&email];
    let q:String = conn.prepare(q, &args);
    let r = conn.query(&q);
    let row = r.result.first().unwrap();
    let id: String = row.get("id").unwrap();
    id
}

pub async fn validate_password(email: String, password: String) -> bool {
    let res_pass = get_user_pass(email).await;
    if res_pass.is_empty() {
        return false;
    }
    check_password(&password, &res_pass)
}