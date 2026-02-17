use axum::{routing::{get, post}, Router};

mod domain;
mod service;
mod config;

use crate::service::dino_service::{create_dino, get_dino};

#[tokio::main]
async fn main() {
    let pool = config::db::db_pool().await;

    // Construimos la App inyectando el pool de conexión
    let app = Router::new()
        .route("/dinos/:id", get(get_dino))
        .route("/dinos", post(create_dino))
        .with_state(pool); // Inyección de dependencias explícita

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Servidor corriendo en el puerto 8080");
    axum::serve(listener, app).await.unwrap();
}