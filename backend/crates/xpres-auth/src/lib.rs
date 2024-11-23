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

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Route {
    pub id: i32,
    pub start: String,
    pub end: String,
    pub data: Vec<u8>
}

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Ticket {
    pub id: i32,
    pub owner_id: i32,
    pub route_id: i32,
    pub data: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct AdminUser(User);

impl Debug for AdminUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Admin{:?}", self.0)
    }
}

pub struct Database {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}
