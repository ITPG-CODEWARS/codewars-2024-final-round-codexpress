mod sqlite;

use crate::prelude::*;

#[rocket::async_trait]
pub trait DBConnection: Send + Sync {
    async fn init(&self) -> Result<()>;

    // User-related methods
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()>;
    async fn delete_user_by_email(&self, email: &str) -> Result<()>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<User>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;

    // Route-related methods
    async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<(), Error>;
    async fn delete_route_by_id(&self, id: i32) -> Result<()>;
    async fn delete_route(&self, start: &str, end: &str) -> Result<()>;
    async fn get_route_by_id(&self, id: i32) -> Result<Route>;
    async fn get_route_by_start(&self, start: &str) -> Result<Vec<Route>>;

    async fn create_ticket(&self, user_id: i32, route_id: i32, data: Vec<u8>) -> Result<(), Error>;
    async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket>;
    async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>>;
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for std::sync::Arc<T> {
    async fn init(&self) -> Result<()> {
        T::init(self).await
    }

    // User-related methods
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        T::create_user(self, email, hash, is_admin).await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        T::update_user(self, user).await
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        T::delete_user_by_id(self, user_id).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        T::delete_user_by_email(self, email).await
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        T::get_user_by_id(self, user_id).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        T::get_user_by_email(self, email).await
    }

    // Route-related methods
    async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<(), Error> {
        T::create_route(self, start, end, data).await
    }
    async fn delete_route_by_id(&self, id: i32) -> Result<()> {
        T::delete_route_by_id(self, id).await
    }
    async fn delete_route(&self, start: &str, end: &str) -> Result<()> {
        T::delete_route(self, start, end).await
    }
    async fn get_route_by_id(&self, id: i32) -> Result<Route> {
        T::get_route_by_id(self, id).await
    }
    async fn get_route_by_start(&self, start: &str) -> Result<Vec<Route>> {
        T::get_route_by_start(self, start).await
    }

    // Ticket-related methods
    async fn create_ticket(&self, user_id: i32, route_id: i32, data: Vec<u8>) -> Result<(), Error> {
        T::create_ticket(self, user_id, route_id, data).await
    }
    async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket> {
        T::get_ticket_by_id(self, ticket_id).await
    }
    async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>> {
        T::get_tickets_by_user(self, user_id).await
    }
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for tokio::sync::Mutex<T> {
    async fn init(&self) -> Result<()> {
        self.init().await
    }

    // User-related methods
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        self.lock().await.create_user(email, hash, is_admin).await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.lock().await.update_user(user).await
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        self.lock().await.delete_user_by_id(user_id).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        self.lock().await.delete_user_by_email(email).await
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        self.lock().await.get_user_by_id(user_id).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.lock().await.get_user_by_email(email).await
    }

    // Route-related methods
    async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<(), Error> {
        self.lock().await.create_route(start, end, data).await
    }
    async fn delete_route_by_id(&self, id: i32) -> Result<()> {
        self.lock().await.delete_route_by_id(id).await
    }
    async fn delete_route(&self, start: &str, end: &str) -> Result<()> {
        self.lock().await.delete_route(start, end).await
    }
    async fn get_route_by_id(&self, id: i32) -> Result<Route> {
        self.lock().await.get_route_by_id(id).await
    }
    async fn get_route_by_start(&self, start: &str) -> Result<Vec<Route>> {
        self.lock().await.get_route_by_start(start).await
    }

    // Ticket-related methods
    async fn create_ticket(&self, user_id: i32, route_id: i32, data: Vec<u8>) -> Result<(), Error> {
        self.lock()
            .await
            .create_ticket(user_id, route_id, data)
            .await
    }
    async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket> {
        self.lock().await.get_ticket_by_id(ticket_id).await
    }
    async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>> {
        self.lock().await.get_tickets_by_user(user_id).await
    }
}
