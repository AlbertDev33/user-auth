use std::fmt::Debug;
use std::future::{ready, Ready};
use std::pin::Pin;

use actix_web::HttpRequest;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpResponse,
};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use futures::Future;
use serde::{Deserialize, Serialize};

use crate::database::models::{CustomResult, Pool, User};
use crate::{error_handling::errors::ServiceError, utils::hash};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct AuthMiddleware;
#[derive(Debug)]
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S:Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // println!("{request:?}");
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.

        // Don't forward to `/login` if we are already on `/login`.
        // let (request, _pl) = request.into_parts();

        // let response = HttpResponse::Found()
        //     .insert_header(("X-Custom-Header", "Hello custom header"))
        //     .finish()
        //     .map_into_right_body::<S>();
        // let response = HttpResponse::Found()
        //     .insert_header((http::header::LOCATION, "/login"))
        //     .finish()
        //     // constructed responses map to "right" body
        //     .map_into_right_body();
        let foo = self.service.call(request);
        return Box::pin(async move {
            let res = foo.await?;
            Ok(res)
        });
    }
}

pub async fn auth_middleware(
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> CustomResult<HttpResponse> {
    let header = req.headers();
    let email = String::from(
        header
            .get("email")
            .expect("Email and password is required.")
            .to_str()
            .unwrap(),
    );
    let password = String::from(
        header
            .get("password")
            .expect("Email and password is required.")
            .to_str()
            .unwrap(),
    );

    let user = user_login(email, password, pool).await?;
    return Ok(HttpResponse::Ok().json(user));
}

async fn user_login(email: String, password: String, pool: web::Data<Pool>) -> CustomResult<User> {
    let (user_result, _is_valid) = {
        let credentials = LoginRequest { email, password };
        let user = find_user(credentials.clone(), pool)?;
        let is_valid = hash::verify(&user.hash, &credentials.password)?;
        (user, is_valid)
    };

    return Ok(user_result);
}

fn find_user(user_data: LoginRequest, pool: web::Data<Pool>) -> CustomResult<User> {
    use crate::database::schema::users::dsl::{email, users};

    let db_connection = &mut pool.get()?;

    let user = users
        .filter(email.eq(&user_data.email))
        .get_result::<User>(db_connection)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid password or email".into()));
    return user;
}
