#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    web::{self},
    App, HttpServer,
};
use std::io::Result;
use time;

mod constants;
mod database;
mod error_handling;
mod handlers;
mod utils;

use database::connection_pool::database_connection_pool;
use handlers::{invitation_handler::post_invitation, register_handler::register_user};

use utils::hash::SECRET_KEY;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (pool, domain) = database_connection_pool();

    return HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(time::Duration::days(1))
                    .secure(false),
            ))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(web::resource("/invitation").route(web::post().to(post_invitation)))
                    .service(
                        web::resource("/user/{invitation_id}").route(web::post().to(register_user)),
                    ),
            )
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await;
}
