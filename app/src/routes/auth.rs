use crate::{routes::error::Error};

use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{fmt};


const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";

#[derive(Clone, PartialEq, Debug)]
pub enum Role {
    User,
    Admin,
}


impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}



#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

// pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
//     headers_cloned()
//         .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
//         .and_then(authorize)
// }

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