use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Producto {
    pub id: i64,
    pub nombre: String,
    pub descripcion: String,
    pub activo: bool,
    pub categoria_id: i64,
    pub marca_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub unidad_id: i32,
    pub barras: String,
    pub contenido: String,
    pub caracteristicas: String,
    pub fraccionable: bool,
    pub cantidad: i32,
}
impl Producto {
    pub fn new() -> Producto {
        Producto {
            id: 0,
            nombre: String::new(),
            descripcion: String::new(),
            activo: true,
            categoria_id: 0,
            marca_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            unidad_id: 0,
            barras: String::new(),
            contenido: String::new(),
            caracteristicas: String::new(),
            fraccionable: false,
            cantidad: 0,
        }
    }
}
