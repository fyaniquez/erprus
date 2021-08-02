use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct Caja {
    #[serde(default)]
    pub id: i64,
    pub descripcion: String,
    #[serde(default = "default_activa")]
    pub activa: bool,
    #[serde(default = "default_sucursal")]
    pub sucursal_id: i64,
    pub desde: String,
    pub hasta: String,
    #[serde(default = "default_date")]
    pub created_at: NaiveDateTime,
    #[serde(default = "default_date")]
    pub updated_at: NaiveDateTime,
    pub numero: String,
}
impl Caja {
    pub fn vacio() -> Caja {
        Caja {
            id: 0,
            descripcion: String::from(""),
            activa: true,
            sucursal_id: 0,
            desde: String::from(""),
            hasta: String::from(""),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            numero: String::from(""),
        }
    }
}
fn default_date() -> NaiveDateTime {
    Utc::now().naive_utc()
}
fn default_sucursal() -> i64 {
    1
}
fn default_activa() -> bool {
    true
}
