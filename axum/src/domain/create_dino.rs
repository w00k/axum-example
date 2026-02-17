use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateDino {
    pub nombre: String,
    pub especie: String,
}
