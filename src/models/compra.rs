/// modelo compra
/// autor: fyaniquez
/// fecha: 2021-08-14
///
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde; // 1.0.85
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Compra<'a> {
    pub id: i64,
    pub fecha: NaiveDateTime,
    pub total: i32,
    pub pago: i32,
    pub factura: bool,
    pub numero: &'a str,
    pub observaciones: &'a str,
    pub distribuidora_id: i64,
    pub empleado_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl<'a> Compra<'a> {
    pub fn new() -> Compra<'a> {
        Compra {
            id: 0,
            fecha: Utc::now().naive_utc(),
            total: 0,
            pago: 0,
            factura: false,
            numero: "",
            observaciones: "",
            distribuidora_id: 0,
            empleado_id: 0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
    pub fn from_parametros_form(parametros: &'a CompraForm) -> Compra<'a> {
        Compra {
            id: 0,
            fecha: Utc::now().naive_utc(),
            total: parametros.total,
            pago: parametros.pago,
            factura: parametros.factura,
            numero: parametros.numero,
            observaciones: parametros.observaciones,
            distribuidora_id: parametros.distribuidora_id,
            empleado_id: get_empleado(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

// parametros obtenidos del formulario
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CompraForm<'a> {
    pub total: i32,
    pub pago: i32,
    pub factura: bool,
    pub numero: &'a str,
    pub observaciones: &'a str,
    pub distribuidora_id: i64,
    pub producto_id: Vec<i64>,
    pub cantidad: Vec<i32>,
    pub costo: Vec<i32>,
    pub subtotal: Vec<i32>,
    pub vencimiento: Vec<NaiveDateTime>,
}

fn get_empleado() -> i64 {
    1
}
