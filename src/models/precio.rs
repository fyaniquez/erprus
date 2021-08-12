use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Precio {
    pub id: i64,
    pub precio: i32,
    pub costo: i32,
    pub catalogo_id: i64,
    pub producto_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Precio {
    pub fn new() -> Precio {
        Precio {
            id: 0,
            precio: 0,
            costo: 0,
            catalogo_id: 0,
            producto_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductoPrecio {
    pub id: i64,
    pub nombre: String,
    pub fraccionable: bool,
    pub cantidad: i32,
    pub precio: i32,
    pub costo: i32,
}
