use rocket::{form::*, get, post, response::Redirect, routes, State};
use xpres_auth::{prelude::Error, *};
use serde_json::json;
use sqlx::*;

use std::result::Result;
use std::*;
#[get("/login")]
fn get_login() -> String {
    "Login Page".to_string()
}

#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
async fn get_signup() -> String {
    "Signup page".to_string()
}

#[post("/signup", data = "<form>")]
async fn post_signup(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;

    Ok(Redirect::to("/"))
}

#[get("/")]
async fn index(user: Option<User>) -> String {
    format!("User: {}",  json!({ "user": user }))
}

#[get("/logout")]
fn logout(auth: Auth<'_>) -> Result<String, Error> {
    auth.logout()?;
    Ok("Logout success".to_string())
}
#[get("/delete")]
async fn delete(auth: Auth<'_>) -> Result<String, Error> {
    auth.delete().await?;
    Ok("DEleted successfully".to_string())
}

#[get("/show_all_users")]
async fn show_all_users(conn: &State<SqlitePool>, user: Option<User>) -> Result<String, Error> {
    let users: Vec<User> = query_as("select * from users;").fetch_all(&**conn).await?;
    println!("{:?}", users);
    // Ok(Template::render(
    //     "users",
    //     json!({"users": users, "user": user}),
    // ))
    Ok(format!("Users: {}", json!({"users": users, "user": user})))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = SqlitePool::connect("database.db").await?;
    let users: Users = conn.clone().into();
    users.create_table().await?;

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_login,
                post_signup,
                get_signup,
                post_login,
                logout,
                delete,
                show_all_users,
            ],
        )
        .manage(conn)
        .manage(users)
        // .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
    Ok(())
}
