use dotenvy::dotenv;
use sqlx::PgPool;

pub async fn db_pool() -> PgPool {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&db_url).await.expect("No se pudo conectar a la DB")
}
