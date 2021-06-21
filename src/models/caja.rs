use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct Caja {
    #[serde(default)]
    pub id: i64,
    pub descripcion: String,
    pub activa: bool,
    #[serde(default = "default_sucursal")]
    pub sucursal_id: i64,
    pub desde: NaiveDateTime,
    pub hasta: NaiveDateTime,
    #[serde(default = "default_date")]
    pub created_at: NaiveDateTime,
    #[serde(default = "default_date")]
    pub updated_at: NaiveDateTime,
}
impl Caja {
    pub fn vacio() -> Caja {
        Caja {
            id: 0,
            descripcion: String::from(""),
            activa: false,
            sucursal_id: 0,
            desde: Utc::now().naive_utc(),
            hasta: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
fn default_date() -> NaiveDateTime {
    Utc::now().naive_utc()
}
fn default_sucursal() -> i64 {
    1
}
