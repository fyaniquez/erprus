use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::Type)]
pub struct Caja {
    pub id: i64,
    pub descripcion: Option<String>,
    pub activa: Option<bool>,
    pub sucursal_id: i64,
    pub desde: NaiveDateTime,
    pub hasta: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
