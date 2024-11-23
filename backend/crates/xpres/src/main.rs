use rocket::{form::*, get, post, response::Redirect, routes, State};
use xpres_auth::{prelude::Error, *};
use serde_json::json;
use sqlx::*;

use std::result::Result;
use std::*;

#[post("/api/login", data = "<form>")]
async fn login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/"))
}



#[post("/api/register", data = "<form>")]
async fn register(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;

    Ok(Redirect::to("/"))
}

#[get("/api/logout")]
fn logout(auth: Auth<'_>) -> Result<String, Error> {
    auth.logout()?;
    Ok("Logout success".to_string())
}
#[get("/api/delete")]
async fn delete(auth: Auth<'_>) -> Result<String, Error> {
    auth.delete().await?;
    Ok("Deleted successfully".to_string())
}

#[get("/api/userlist")]
async fn userlist(conn: &State<SqlitePool>, user: Option<User>) -> Result<String, Error> {
    let users: Vec<User> = query_as("select * from users;").fetch_all(&**conn).await?;
    println!("{:?}", users);
    Ok(format!("Users: {}", json!({"users": users, "user": user})))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = SqlitePool::connect("sqlite::memory:").await?;
    let users: Users = conn.clone().into();
    users.create_table().await?;

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                register,
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



// #[get("/login")]
// fn get_login() -> String {
//     "Login Page".to_string()
// }


// #[get("/signup")]
// async fn get_signup() -> String {
//     "Signup page".to_string()
// }


// #[get("/")]
// async fn index(user: Option<User>) -> String {
//     format!("User: {}",  json!({ "user": user }))
// }