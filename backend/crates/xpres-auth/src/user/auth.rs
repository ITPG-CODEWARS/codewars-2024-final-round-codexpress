use crate::prelude::*;
use regex::Regex;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;
use serde_json::json;
use std::time::Duration;

pub fn validate_email(email: &String) -> bool {
    let expr = Regex::new("^[\\w\\.-]+@[\\w\\.-]+\\.[a-zA-Z]{2,6}$");

    if let Ok(reg_ex) = expr {
        return reg_ex.is_match(&email);
    } else {
        return false;
    }
}

#[allow(missing_docs)]
pub struct Auth<'a> {
    pub users: &'a State<Database>,
    pub cookies: &'a CookieJar<'a>,
    pub session: Option<Session>,
}

#[async_trait]
impl<'r> FromRequest<'r> for Auth<'r> {
    type Error = Error;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Auth<'r>, Error> {
        let session: Option<Session> = if let Outcome::Success(users) = req.guard().await {
            Some(users)
        } else {
            None
        };

        let users: &State<Database> = if let Outcome::Success(users) = req.guard().await {
            users
        } else {
            return Outcome::Error((Status::InternalServerError, Error::UnmanagedStateError));
        };

        Outcome::Success(Auth {
            users,
            session,
            cookies: req.cookies(),
        })
    }
}

impl<'a> Auth<'a> {
    #[throws(Error)]
    pub async fn login(&self, form: &Login) {
        let key = self.users.login(form).await?;
        let user = self.users.get_by_email(&form.email.to_lowercase()).await?;
        let session = Session {
            id: user.id,
            email: user.email,
            auth_key: key,
            time_stamp: now(),
        };
        let to_str = format!("{}", json!(session));
        self.cookies.add_private(Cookie::new("xpres-auth", to_str));
    }

    #[throws(Error)]
    pub async fn login_for(&self, form: &Login, time: Duration) {
        let key = self.users.login_for(form, time).await?;
        let user = self.users.get_by_email(&form.email.to_lowercase()).await?;

        let session = Session {
            id: user.id,
            email: user.email,
            auth_key: key,
            time_stamp: now(),
        };
        let to_str = format!("{}", json!(session));
        let cookie = Cookie::new("xpres-auth", to_str);
        self.cookies.add_private(cookie);
    }

    #[throws(Error)]
    pub async fn signup(&self, form: &Signup) {
        self.users.signup(form).await?;
    }

    #[throws(Error)]
    pub async fn signup_for(&self, form: &Signup, time: Duration) {
        self.users.signup(form).await?;
        self.login_for(&form.clone().into(), time).await?;
    }

    pub fn is_auth(&self) -> bool {
        if let Some(session) = &self.session {
            self.users.is_auth(session)
        } else {
            false
        }
    }

    pub async fn get_user(&self) -> Option<User> {
        if !self.is_auth() {
            return None;
        }
        let id = self.session.as_ref()?.id;
        if let Ok(user) = self.users.get_by_id(id).await {
            Some(user)
        } else {
            None
        }
    }

    #[throws(Error)]
    pub fn logout(&self) {
        let session = self.get_session()?;
        self.users.logout(session)?;
        self.cookies.remove_private(Cookie::build("xpres-auth"));
    }

    #[throws(Error)]
    pub async fn delete(&self) {
        if self.is_auth() {
            let session = self.get_session()?;
            self.users.delete(session.id).await?;
            self.cookies.remove_private("xpres-auth");
        } else {
            throw!(Error::UnauthenticatedError)
        }
    }

    pub async fn change_password(&self, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_auth() {
            let session = self.get_session()?;
            let mut user = self.users.get_by_id(session.id).await?;
            user.set_password(password)?;
            self.users.modify(&user).await?;

            Ok(())
        } else {
            throw!(Error::UnauthorizedError)
        }
    }

    pub async fn change_email(&self, email: String) -> Result<(), Error> {
        if self.is_auth() {
            if !validate_email(&email) {
                return Err(Error::InvalidEmailAddressError);
            }
            let session = self.get_session()?;
            let mut user = self.users.get_by_id(session.id).await?;
            user.email = email.to_lowercase();
            self.users.modify(&user).await?;
            return Ok(());
        } else {
            return Err(Error::UnauthorizedError);
        }
    }

    pub fn get_session(&self) -> Result<&Session> {
        let session = self.session.as_ref().ok_or(Error::UnauthenticatedError)?;
        Ok(session)
    }

    #[throws(Error)]
    pub async fn compare_password(&self, password: &str) -> bool {
        if self.is_auth() {
            let session = self.get_session()?;
            let user: User = self.users.get_by_id(session.id).await?;
            user.compare_password(password)?
        } else {
            throw!(Error::UnauthorizedError)
        }
    }
}

#[cfg(test)]
mod test {

    use super::validate_email;

    #[test]
    fn test_validate_email() {
        let good_mail = String::from("some.example@gmail.com");
        let bad_mail = String::from("@fak,.r");
        assert!(validate_email(&good_mail));
        assert!(!(validate_email(&bad_mail)));
    }
}
