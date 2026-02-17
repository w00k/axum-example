use axum::{extract::{State, Path}, routing::{get, post}, Json, Router, http::StatusCode};
use sqlx::PgPool;

use dotenvy::dotenv;

mod domain;
use crate::domain::{create_dino::CreateDino, dino::Dino};

// Handler para obtener un dinosaurio
async fn get_dino(
    State(pool): State<PgPool>, 
    Path(id): Path<i32>
) -> Result<Json<Dino>, StatusCode> {
    // SQLx comprueba que la consulta sea válida
    let dino = sqlx::query_as::<_, Dino>("SELECT id, nombre, especie FROM dinosaurios WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(dino))
}

// Handler para crear uno (Manejo de errores básico)
async fn create_dino(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateDino>,
) -> StatusCode {
    let result = sqlx::query("INSERT INTO dinosaurios (nombre, especie) VALUES ($1, $2)")
        .bind(payload.nombre)
        .bind(payload.especie)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

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