use actix_web::{web, HttpRequest};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

// type Users = Arc<RwLock<HashMap<String, User>>>;
use crate::database::models::{CustomResult, Pool, User};
use crate::{error_handling::errors::ServiceError, utils::hash};

// #[derive(Clone)]
// pub struct User {
//     pub id: Uuid,
//     pub email: String,
//     pub password: String,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn user_login(req: HttpRequest, pool: web::Data<Pool>) -> CustomResult<User> {
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
