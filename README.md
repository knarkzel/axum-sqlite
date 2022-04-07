# axum-sqlite

```bash
cargo add axum-sqlite
```

Provides Sqlite database for `axum`.

```rust
use axum::{Extension, response::Html, routing::get, Router};
use std::net::SocketAddr;
use axum_sqlite::*;
 
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .layer(Database::new(":memory:").unwrap());
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
 
async fn index(Extension(database): Extension<Database>) -> Html<&'static str> {
    let connection = database.connection().unwrap(); // Do stuff with connection
    Html("Hello, sqlite!")
}
```
