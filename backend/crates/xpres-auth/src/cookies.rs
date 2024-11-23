use crate::error;
use crate::prelude::*;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use serde_json::from_str;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Session {
    pub time_stamp: i64,

    pub id: i32,

    pub email: String,

    pub auth_key: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Session, Self::Error> {
        let cookies = request.cookies();

        if let Some(session) = get_session(cookies) {
            Outcome::Success(session)
        } else {
            Outcome::Error((Status::Unauthorized, error::Error::UnauthorizedError))
        }
    }
}
#[throws(as Option)]
fn get_session(cookies: &CookieJar) -> Session {
    let session = cookies.get_private("rocket_auth")?;
    from_str(session.value()).ok()?
}
