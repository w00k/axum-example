use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Dino {
    pub id: i32,
    pub nombre: String,
    pub especie: String,
}

