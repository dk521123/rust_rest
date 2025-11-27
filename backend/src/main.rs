mod models;

use axum::{
    routing::get,
    Router,
    Json as AxumJson,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use uuid::Uuid;
use crate::models::user::User;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", get(list_users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, world!!!!"
}

async fn list_users() -> AxumJson<Vec<User>> {
    let users = vec![
        User { id: Uuid::new_v4(), name: "Alice".into() },
        User { id: Uuid::new_v4(), name: "Bob".into() },
        User { id: Uuid::new_v4(), name: "Charlie".into() },
    ];
    AxumJson(users)
}
