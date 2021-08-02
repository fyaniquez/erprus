/// modelo marca
/// autor: fyaniquez
/// fecha: 2021-07-12 19:22:46.428115562 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Marca {
    pub id: i64,
    pub nombre: String,
    pub fabrica_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Marca {
    pub fn new() -> Marca {
        Marca {
            id: 0,
            nombre: String::new(),
            fabrica_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
