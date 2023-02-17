use std::env::var;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

use crate::constants::constants;
use crate::database::models::Pool;

pub fn database_connection_pool() -> (Pool, String) {
    let database_url = constants();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    return (pool, domain);
}
