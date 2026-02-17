use axum::{extract::{State, Path}, Json, http::StatusCode};
use sqlx::PgPool;
use crate::domain::{create_dino::CreateDino, dino::Dino};

// Handler para obtener un dinosaurio
pub async fn get_dino(
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
pub async fn create_dino(
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
