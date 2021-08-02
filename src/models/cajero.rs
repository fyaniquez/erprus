use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cajero {
    #[serde(default)]
    pub id: i64,
    pub apertura: NaiveDateTime,
    pub cierre: Option<NaiveDateTime>,
    pub inicial: i32,
    pub terminal: i32,
    pub ingresos: i32,
    pub egresos: i32,
    #[serde(default = "default_activo")]
    pub activo: bool,
    pub admin_id: i32,
    pub empleado_id: i64,
    pub caja_id: i64,
    #[serde(default = "default_date")]
    pub created_at: NaiveDateTime,
    #[serde(default = "default_date")]
    pub updated_at: NaiveDateTime,
}
impl Cajero {
    pub fn vacio() -> Cajero {
        Cajero {
            id: 0,
            apertura: Utc::now().naive_utc(),
            cierre: None,
            inicial: 0,
            terminal: 0,
            ingresos: 0,
            egresos: 0,
            activo: true,
            admin_id: 0,
            empleado_id: 0,
            caja_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
fn default_date() -> NaiveDateTime {
    Utc::now().naive_utc()
}
fn default_activo() -> bool {
    true
}
pub fn date_null(fecha: Option<NaiveDateTime>) -> String {
    match fecha {
        Some(f) => f.to_string(),
        None => String::from(""),
    }
}
