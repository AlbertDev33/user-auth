use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::database::models::{Invitation, Pool, SlimUser, User};
use crate::error_handling::errors::ServiceError;
use crate::utils::hash::hash_password;

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || query(invitation_id.into_inner(), user_data.into_inner(), pool))
        .await??;

    return Ok(HttpResponse::Ok().json(&user));
}

fn query(
    invitation_id: String,
    user_data: UserData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::database::schema::invitations::dsl::{email, id, invitations};
    use crate::database::schema::users::dsl::users;

    let uuid_invitation_id = uuid::Uuid::parse_str(&invitation_id)?;
    let db_connection = &mut pool.get()?;

    let inserted_user = invitations
        .filter(id.eq(uuid_invitation_id))
        .filter(email.eq(&user_data.email))
        .load::<Invitation>(db_connection)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation or email".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    let password: String = hash_password(&user_data.password)?;
                    let user = User::from_details(invitation.email, password);
                    let inserted_user: User = diesel::insert_into(users)
                        .values(&user)
                        .get_result(db_connection)?;
                    dbg!(&inserted_user);
                    return Ok(inserted_user.into());
                }
            }
            return Err(ServiceError::BadRequest("Invalid Invitation".into()));
        });

    return inserted_user;
}
