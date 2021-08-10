use crate::models::venta::*;
/// modelo vendido
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.882991891 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vendido {
    pub id: i64,
    pub cantidad: i32,
    pub precio: i32,
    pub subtotal: i32,
    pub producto_id: i64,
    pub venta_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Vendido {
    pub fn new() -> Vendido {
        Vendido {
            id: 0,
            cantidad: 0,
            precio: 0,
            subtotal: 0,
            producto_id: 0,
            venta_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
    pub fn from_parametros_form(venta: &ParametrosForm, i: usize) -> Vendido {
        Vendido {
            id: 0,
            producto_id: venta.producto_id[i],
            cantidad: venta.cantidad[i],
            precio: venta.precio[i],
            subtotal: venta.subtotal[i],
            venta_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
