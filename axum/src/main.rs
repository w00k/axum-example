use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use dotenvy::dotenv;

mod domain;
mod service;

use crate::service::dino_service::{create_dino, get_dino};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("No se pudo conectar a la DB");

    // Construimos la App inyectando el pool de conexión
    let app = Router::new()
        .route("/dinos/:id", get(get_dino))
        .route("/dinos", post(create_dino))
        .with_state(pool); // Inyección de dependencias explícita

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Servidor corriendo en el puerto 8080");
    axum::serve(listener, app).await.unwrap();
}