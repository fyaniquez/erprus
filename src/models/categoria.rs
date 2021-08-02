/// modelo categoria
/// autor: fyaniquez
/// fecha: 2021-07-11 21:43:03.192027186 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Categoria {
    pub id: i64,
    pub nombre: String,
    pub capitulo_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Categoria {
    pub fn new() -> Categoria {
        Categoria {
            id: 0,
            nombre: String::new(),
            capitulo_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
