use actix_web::{web, HttpResponse, HttpRequest};
use diesel::prelude::*;
use serde::Deserialize;

use crate::database::models::{Invitation, Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn post_invitation(
    req: HttpRequest,
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let header = req.headers();
    let app_id = header.get("x_application_id").expect("Invalid key");
    println!("{:?}", app_id);
    web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await??;
    return Ok(HttpResponse::Ok().finish());
}

fn create_invitation(eml: String, pool: web::Data<Pool>) -> Result<(), crate::error_handling::errors::ServiceError> {
    let invitation = dbg!(query(eml, pool)?);
    return Ok(());
}

fn query(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<Invitation, crate::error_handling::errors::ServiceError> {
    use crate::database::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let db_connection = &mut pool.get()?;

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(db_connection)?;

    return Ok(inserted_invitation);
}
