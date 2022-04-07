use r2d2::{Pool, Error};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone)]
pub struct Layer {
    pool: Pool<SqliteConnectionManager>,
}

impl Layer {
    pub fn new(path: &str) -> Result<Self, Error> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }
}

impl<S> tower::Layer<S> for Layer {
    type Service = Service<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Service {
            pool: self.pool.clone(),
            inner,
        }
    }
}

#[derive(Clone)]
pub struct Service<S> {
    pool: Pool<SqliteConnectionManager>,
    inner: S,
}
