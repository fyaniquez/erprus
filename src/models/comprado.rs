/// modelo comprado
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.882991891 -04:00
///
use crate::models::compra::*;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comprado {
    pub id: i64,
    pub cantidad: i32,
    pub costo: i32,
    pub subtotal: i32,
    pub vencimiento: NaiveDateTime,
    pub producto_id: i64,
    pub compra_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comprado {
    pub fn from_parametros_form(compra: &CompraForm, i: usize) -> Comprado {
        Comprado {
            id: 0,
            producto_id: compra.producto_id[i],
            cantidad: compra.cantidad[i],
            costo: compra.costo[i],
            subtotal: compra.subtotal[i],
            vencimiento: compra.vencimiento[i],
            compra_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
