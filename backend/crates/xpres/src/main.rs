use rocket::Responder;
use rocket::{form::*, get, post, response::Redirect, routes, State};
use xpres_auth::{prelude::Error, Datastore, *};
use serde_json::json;
use sqlx::*;
use rocket::response::content::RawHtml;

use std::result::Result;
use std::*;

// ADMIN-RELATED
#[post("/api/admin/register", data = "<form>")]
async fn register_admin(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup_admin(&form).await?;
    auth.login(&form.into()).await?;

    Ok(Redirect::to("/"))
}

// AUTH-RELATED
#[post("/api/user/login", data = "<form>")]
async fn login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/"))
}

#[post("/api/user/register", data = "<form>")]
async fn register(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;

    Ok(Redirect::to("/"))
}

#[get("/api/user/logout")]
fn logout(auth: Auth<'_>) -> Result<String, Error> {
    auth.logout()?;
    Ok("Logout success".to_string())
}
#[get("/api/user/delete")]
async fn delete(auth: Auth<'_>) -> Result<String, Error> {
    auth.delete().await?;
    Ok("Deleted successfully".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = SqlitePool::connect("sqlite::memory:").await?;
    let users: xpres_auth::Datastore = conn.clone().into();
    users.create_table().await?;

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                register,
                index,
                get_login,
                login,
                logout,
                delete,
                userlist,
            ],
        )
        .manage(conn)
        .manage(users)
        .launch()
        .await
        .unwrap();
    Ok(())
}


#[get("/api/userlist")]
async fn userlist(conn: &State<SqlitePool>, user: Option<User>) -> Result<String, Error> {
    let users: Vec<User> = query_as("select * from users;").fetch_all(&**conn).await?;
    println!("{:?}", users);
    Ok(format!("Users: {}", json!({"users": users, "user": user})))
}


#[get("/login")]
fn get_login() -> RawHtml<String> {
    RawHtml(include_str!("index.html").to_string())
}


// #[get("/signup")]
// async fn get_signup() -> String {
//     "Signup page".to_string()
// }


#[get("/")]
async fn index(user: Option<User>) -> String {
    format!("User: {}",  json!({ "user": user }))
}