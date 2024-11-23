mod cookies;
mod db;
mod error;
mod forms;
pub mod prelude;
mod session;
mod user;

#[cfg(test)]
mod tests;

use std::fmt::Debug;

pub use prelude::*;

pub use crate::user::auth::Auth;
pub use cookies::Session;
pub use error::Error;

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct User {
    pub id: i32,
    email: String,
    pub is_admin: bool,
    #[serde(skip_serializing)]
    password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct AdminUser(User);

impl Debug for AdminUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Admin{:?}", self.0)
    }
}

pub struct Users {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}
