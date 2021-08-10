/// controller venta
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:06.537215229 -04:00
///
use crate::handlers::venta::*;
use crate::models::vendido::Vendido;
use crate::models::venta::{ParametrosForm, Venta};
use crate::views::venta::*;
use crate::State;
//use serde_qs::from_str;
use serde_qs::Config;
use sqlx::PgPool;
use tide::http::mime;
use tide::Request;
use tide::{Body, Response};

/// formulario para ingresar nuevo registro
pub async fn ctrl_new(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let errores = String::new();
    let res = view_new(errores).await.unwrap();
    Ok(res)
}

/// api para crear un registro con los datos enviados
pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    // descartado temporalmente hasta saber como interpretar los corchetes codificados
    //let parametros: ParametrosForm = req.body_form().await?;

    // en lugar de la sentencia anterior
    // lee el formulario como string
    let parametros_str = req.body_string().await?;
    // ajusta el conversor a modo no estricto para que interprete los corchetes codificados
    let qs_non_strict = Config::new(5, false);
    // llena la structura con los parametros del string
    let parametros: ParametrosForm = qs_non_strict.deserialize_str(&parametros_str).unwrap();
    // fin en lugar de la sentencia anterior

    // construye la cabecera de la venta a partir de la estructura de parametros
    let venta = Venta::fromParametrosForm(&parametros);

    // construye el detalle de la venta a partir de la estructura de parametros
    let mut vendidos: Vec<Vendido> = Vec::new();
    for i in 0..parametros.producto_id.len() {
        vendidos.push(Vendido::from_parametros_form(&parametros, i));
    }

    // crea la venta
    let res = match hndl_create(&db_pool, venta, vendidos).await {
        Ok(row) => ctrl_new(req).await,
        Err(err) => view_edit(err.to_string()).await,
    };

    Ok(res.unwrap())
}
