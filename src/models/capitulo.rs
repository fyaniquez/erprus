/// modelo capitulo
/// autor: fyaniquez
/// fecha: 2021-07-12 18:54:46.496996494 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Capitulo {
    pub id: i64,
    pub nombre: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub descripcion: String,
}

impl Capitulo {
    pub fn new() -> Capitulo {
        Capitulo {
            id: 0,
            nombre: String::new(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            descripcion: String::new(),
        }
    }
}
