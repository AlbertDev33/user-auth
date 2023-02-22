use argon2::{self, Config};
use std::{env::var, fmt::Error};

use crate::error_handling::errors::ServiceError;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

const SALT: &'static [u8] = b"supersecuresalt";

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };

    return argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        return Err("Error").unwrap();

    });
}

pub fn verify(hash: &str, password: &str) -> Result<bool, Error> {
    return argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(|err| {
        dbg!(err);
        return Err("Error").unwrap();
    })
}