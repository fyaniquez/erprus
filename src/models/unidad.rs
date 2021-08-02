/// modelo unidad
/// autor: fyaniquez
/// fecha: 2021-07-18 21:31:53.585626429 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Unidad {
    pub id: i64,
    pub sigla: String,
    pub nombre: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Unidad {
    pub fn new() -> Unidad {
        Unidad {
            id: 0,
            sigla: String::new(),
            nombre: String::new(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
