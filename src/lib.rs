//! Provides Sqlite database for `axum`.
//!
//! ```rust
//! use axum::{Extension, response::Html, routing::get, Router};
//! use std::net::SocketAddr;
//! use axum_sqlite::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new()
//!         .route("/", get(index))
//!         .layer(Database::new(":memory:").unwrap());
//!     axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! 
//! async fn index(Extension(database): Extension<Database>) -> Html<&'static str> {
//!     let connection = database.connection().unwrap(); // Do stuff with connection
//!     Html("Hello, sqlite!")
//! }
//! ```

use axum::Extension;
use r2d2::{Pool, Error, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(path: &str) -> Result<Extension<Self>, Error> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Extension(Self { pool }))
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
        Ok(self.pool.get()?)
    }
}
