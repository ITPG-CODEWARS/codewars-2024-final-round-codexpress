use super::rand_string;
use crate::db::DBConnection;
use crate::prelude::*;

impl Database {
    #[throws(Error)]
    pub async fn open_sqlite(path: &str) -> Self {
        let conn = sqlx::SqlitePool::connect(path).await?;
        let users: Database = conn.into();
        users.create_table().await?;
        users
    }

    pub async fn create_table(&self) -> Result<(), Error> {
        self.conn.init().await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<User, Error> {
        self.conn.get_user_by_email(email).await
    }

    pub async fn get_by_id(&self, user_id: i32) -> Result<User, Error> {
        self.conn.get_user_by_id(user_id).await
    }

    pub async fn create_user(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> Result<(), Error> {
        let password = password.as_bytes();
        let salt = rand_string(30);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, salt.as_bytes(), &config).unwrap();
        self.conn.create_user(email, &hash, is_admin).await?;

        Ok(())
    }

    #[throws(Error)]
    pub async fn delete(&self, id: i32) {
        self.sess.remove(id)?;
        self.conn.delete_user_by_id(id).await?;
    }

    #[throws(Error)]
    pub async fn modify(&self, user: &User) {
        self.conn.update_user(user).await?;
    }

    // Route-related methods
    pub async fn create_route(&self, start: &str, end: &str, data: Vec<u8>) -> Result<(), Error> {
        self.conn.create_route(start, end, data).await?;
        Ok(())
    }

    pub async fn delete_route_by_id(&self, route_id: i32) -> Result<(), Error> {
        self.conn.delete_route_by_id(route_id).await?;
        Ok(())
    }

    pub async fn delete_route_by_start_end(&self, start: &str, end: &str) -> Result<(), Error> {
        self.conn.delete_route(start, end).await?;
        Ok(())
    }

    pub async fn get_route_by_id(&self, route_id: i32) -> Result<Route, Error> {
        self.conn.get_route_by_id(route_id).await
    }

    pub async fn get_routes_by_start(&self, start: &str) -> Result<Vec<Route>, Error> {
        self.conn.get_route_by_start(start).await
    }

    // Ticket-related methods
    pub async fn create_ticket(
        &self,
        user_id: i32,
        route_id: i32,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self.conn.create_ticket(user_id, route_id, data).await?;
        Ok(())
    }

    pub async fn get_ticket_by_id(&self, ticket_id: i32) -> Result<Ticket, Error> {
        self.conn.get_ticket_by_id(ticket_id).await
    }

    pub async fn get_tickets_by_user(&self, user_id: i32) -> Result<Vec<Ticket>, Error> {
        self.conn.get_tickets_by_user(user_id).await
    }
}

impl<Conn: 'static + DBConnection> From<Conn> for Database {
    fn from(db: Conn) -> Database {
        Database {
            conn: Box::from(db),
            sess: Box::new(chashmap::CHashMap::new()),
        }
    }
}

impl<T0: 'static + DBConnection, T1: 'static + SessionManager> From<(T0, T1)> for Database {
    fn from((db, ss): (T0, T1)) -> Database {
        Database {
            conn: Box::from(db),
            sess: Box::new(ss),
        }
    }
}
