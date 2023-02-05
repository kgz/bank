
use argon2::{self, Config};
use rand::{distributions::Alphanumeric, Rng, thread_rng};

pub fn one_way_encrypt(password: &str) -> String {
    let password = password.as_bytes();
    let salt: String  = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(|x| x as char)
    .collect();
    let salt = salt.as_bytes();
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    hash
}