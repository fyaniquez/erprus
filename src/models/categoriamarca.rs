
/// modelo categoriamarca
/// autor: fyaniquez
/// fecha: 2021-07-17 21:33:19.163708322 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Categoriamarca {
    pub id: i64,
pub categoria_id: i64,
pub marca_id: i64,
pub created_at: NaiveDateTime,
pub updated_at: NaiveDateTime,

}
    
