mod sql;

use crate::prelude::{Result, *};
use rocket::async_trait;
use sql::*;
use tokio::sync::Mutex;

use sqlx::{sqlite::SqliteConnection, *};
#[async_trait]
impl DBConnection for Mutex<SqliteConnection> {
    // User-related methods
    async fn init(&self) -> Result<()> {
        let mut db = self.lock().await;
        query(CREATE_TABLE).execute(&mut *db).await?;
        println!("Table created");
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let mut db = self.lock().await;
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(is_admin)
            .execute(&mut *db)
            .await?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let mut db = self.lock().await;
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(&mut *db)
            .await?;
        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID)
            .bind(user_id)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL)
            .bind(email)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let mut db = self.lock().await;
        let user = query_as(SELECT_BY_ID)
            .bind(user_id)
            .fetch_one(&mut *db)
            .await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let mut db = self.lock().await;
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(&mut *db)
            .await?;
        Ok(user)
    }

    // Route-related methods
    async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<()> {
        let mut db = self.lock().await;
        query(INSERT_ROUTE)
            .bind(start)
            .bind(end)
            .bind(data)
            .execute(&mut *db)
            .await?;
        Ok(())
    }

    async fn delete_route_by_id(&self, id: i32) -> Result<()> {
        query(REMOVE_ROUTE_BY_ID)
            .bind(id)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }

    async fn delete_route(&self, start: &str, end: &str) -> Result<()> {
        query(REMOVE_ROUTE_BY_START_END)
            .bind(start)
            .bind(end)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }

    async fn get_route_by_id(&self, id: i32) -> Result<Route> {
        let mut db = self.lock().await;
        let route = query_as(SELECT_ROUTE_BY_ID)
            .bind(id)
            .fetch_one(&mut *db)
            .await?;
        Ok(route)
    }

    async fn get_route_by_start(&self, start: &str) -> Result<Vec<Route>> {
        let mut db = self.lock().await;
        let routes = query_as(SELECT_ROUTE_BY_START)
            .bind(start)
            .fetch_all(&mut *db)
            .await?;
        Ok(routes)
    }

    // Ticket-related methods
    async fn create_ticket(&self, user_id: i32, route_id: i32, data: Vec<u8>) -> Result<()> {
        let mut db = self.lock().await;
        query(INSERT_TICKET)
            .bind(user_id)
            .bind(route_id)
            .bind(data)
            .execute(&mut *db)
            .await?;
        Ok(())
    }

    async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket> {
        let mut db = self.lock().await;
        let ticket = query_as(SELECT_TICKET_BY_ID)
            .bind(ticket_id)
            .fetch_one(&mut *db)
            .await?;
        Ok(ticket)
    }

    async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>> {
        let mut db = self.lock().await;
        let tickets = query_as(SELECT_TICKET_BY_OWNER_ID)
            .bind(user_id)
            .fetch_all(&mut *db)
            .await?;
        Ok(tickets)
    }
}


#[async_trait]
impl DBConnection for SqlitePool {
    // User-related methods
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE).execute(self).await?;
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(self)
            .await?;
        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL).bind(email).execute(self).await?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(self)
            .await?;
        Ok(user)
    }

    // Route-related methods
    async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<()> {
        query(INSERT_ROUTE)
            .bind(start)
            .bind(end)
            .bind(data)
            .execute(self)
            .await?;
        Ok(())
    }

    async fn delete_route_by_id(&self, id: i32) -> Result<()> {
        query(REMOVE_ROUTE_BY_ID).bind(id).execute(self).await?;
        Ok(())
    }

    async fn delete_route(&self, start: &str, end: &str) -> Result<()> {
        query(REMOVE_ROUTE_BY_START_END)
            .bind(start)
            .bind(end)
            .execute(self)
            .await?;
        Ok(())
    }

    async fn get_route_by_id(&self, id: i32) -> Result<Route> {
        let route = query_as(SELECT_ROUTE_BY_ID).bind(id).fetch_one(self).await?;
        Ok(route)
    }

    async fn get_route_by_start(&self, start: &str) -> Result<Vec<Route>> {
        let routes = query_as(SELECT_ROUTE_BY_START)
            .bind(start)
            .fetch_all(self)
            .await?;
        Ok(routes)
    }

    // Ticket-related methods
    async fn create_ticket(&self, user_id: i32, route_id: i32, data: Vec<u8>) -> Result<()> {
        query(INSERT_TICKET)
            .bind(user_id)
            .bind(route_id)
            .bind(data)
            .execute(self)
            .await?;
        Ok(())
    }

    async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket> {
        let ticket = query_as(SELECT_TICKET_BY_ID)
            .bind(ticket_id)
            .fetch_one(self)
            .await?;
        Ok(ticket)
    }

    async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>> {
        let tickets = query_as(SELECT_TICKET_BY_OWNER_ID)
            .bind(user_id)
            .fetch_all(self)
            .await?;
        Ok(tickets)
    }
}
