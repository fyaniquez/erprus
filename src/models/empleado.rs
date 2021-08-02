use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Empleado {
    pub id: i64,
    pub desde: NaiveDateTime,
    pub hasta: NaiveDateTime,
    pub activo: bool,
    pub usuario_id: i64,
    pub grupo_id: i64,
    pub sucursal_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl Empleado {
    pub fn vacio() -> Empleado {
        Empleado {
            id: 0,
            desde: Utc::now().naive_utc(),
            hasta: Utc::now().naive_utc(),
            activo: true,
            usuario_id: 1,
            grupo_id: 1,
            sucursal_id: 1,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
