use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]

pub struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
    pub(crate) last_login_attempt: String,
}

pub struct _Users {
    users: Vec<User>,
}