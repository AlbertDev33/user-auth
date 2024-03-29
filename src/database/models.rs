use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error_handling::errors::ServiceError;

use super::schema::*;
// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type CustomResult<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        return User {
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        let time_now = chrono::Local::now().naive_local();
        let duration = chrono::Duration::hours(24);
        
        let expires_at = time_now + duration;

        return Invitation {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at,
        };
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        return SlimUser { email: user.email };
    }
}
