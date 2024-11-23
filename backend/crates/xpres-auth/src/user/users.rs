use super::rand_string;
use crate::db::DBConnection;
use crate::prelude::*;

impl Users {
    #[throws(Error)]
    pub async fn open_sqlite(path: &str) -> Self {
        let conn = sqlx::SqlitePool::connect(path).await?;
        let users: Users = conn.into();
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
}

impl<Conn: 'static + DBConnection> From<Conn> for Users {
    fn from(db: Conn) -> Users {
        Users {
            conn: Box::from(db),
            sess: Box::new(chashmap::CHashMap::new()),
        }
    }
}

impl<T0: 'static + DBConnection, T1: 'static + SessionManager> From<(T0, T1)> for Users {
    fn from((db, ss): (T0, T1)) -> Users {
        Users {
            conn: Box::from(db),
            sess: Box::new(ss),
        }
    }
}
