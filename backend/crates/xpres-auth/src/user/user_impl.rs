use super::auth::{validate_email, Auth};
use super::rand_string;

use crate::error;
use crate::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

impl User {
    pub fn set_password(&mut self, new: &str) -> Result<(), Box<dyn std::error::Error>> {
        crate::forms::is_secure(new)?;
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, salt.as_bytes(), &config)?;
        self.password = hash;
        Ok(())
    }

    pub fn compare_password(&self, password: &str) -> Result<bool, argon2::Error> {
        verify_encoded(&self.password, password.as_bytes())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: String) -> Result<(), Error> {
        if validate_email(&email) {
            self.email = email.to_lowercase();
            Ok(())
        } else {
            throw!(Error::InvalidEmailAddressError)
        }
    }
}

use std::fmt::{self, Debug};

impl Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ id: {:?}, email: {:?}, is_admin: {:?}, password: \"*****\" }}",
            self.id, self.email, self.is_admin
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Error> {
        use rocket::outcome::Outcome::*;
        let guard = request.guard().await;
        let auth: Auth = match guard {
            Success(auth) => auth,
            Error(x) => return Error(x),
            Forward(x) => return Forward(x),
        };
        if let Some(user) = auth.get_user().await {
            Outcome::Success(user)
        } else {
            Outcome::Error((Status::Unauthorized, error::Error::UserNotFoundError))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<AdminUser, Error> {
        use rocket::outcome::Outcome::*;
        let guard = request.guard().await;
        let auth: Auth = match guard {
            Success(auth) => auth,
            Error(x) => return Error(x),
            Forward(x) => return Forward(x),
        };
        if let Some(user) = auth.get_user().await {
            if user.is_admin {
                return Outcome::Success(AdminUser(user));
            }
        }
        Outcome::Error((Status::Unauthorized, error::Error::UnauthorizedError))
    }
}

use argon2::verify_encoded;
use std::{ops::*, result};

impl Deref for AdminUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AdminUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl std::convert::TryFrom<User> for AdminUser {
    type Error = Error;
    fn try_from(value: User) -> Result<Self> {
        if value.is_admin {
            Ok(AdminUser(value))
        } else {
            Err(Error::UnauthorizedError)
        }
    }
}
