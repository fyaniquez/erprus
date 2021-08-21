/// modelo venta
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:06.536805665 -04:00
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde; // 1.0.85
use serde::de::{self, MapAccess, Visitor}; // 1.0.85
use serde::{Deserialize, Serialize};
//use serde_derive::{Deserialize, Serialize}; // 1.0.85
use serde_qs::from_str;

#[derive(Serialize, Deserialize)]
pub struct Venta {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: i32,
    pub pago: i32,
    pub cliente_id: i64,
    pub cajero_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Venta {
    pub fn new() -> Venta {
        Venta {
            id: 0,
            fecha: Utc::now().naive_utc(),
            total: 0,
            pago: 0,
            cliente_id: 0,
            cajero_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
    pub fn from_parametros_form(parametros: &ParametrosForm) -> Venta {
        Venta {
            id: 0,
            fecha: Utc::now().naive_utc(),
            total: parametros.total,
            pago: parametros.pago,
            cliente_id: parametros.cliente_id,
            cajero_id: 2,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

// parametros obtenidos del formulario
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VendidosForm {
    pub producto_id: i64,
    pub cantidad: i32,
    pub precio: i32,
    pub subtotal: i32,
}

// parametros obtenidos del formulario
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParametrosForm {
    pub total: i32,
    pub pago: i32,
    pub cliente_id: i64,
    pub producto_id: Vec<i64>,
    pub cantidad: Vec<i32>,
    pub precio: Vec<i32>,
    pub subtotal: Vec<i32>,
}
