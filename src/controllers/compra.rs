/// controller compra
/// autor: fyaniquez
/// fecha: 2021-08-14 23:02:06.537215229 -04:00
///
use crate::handlers::compra::hndl_create;
use crate::models::compra::{Compra, CompraForm};
use crate::models::comprado::Comprado;
use crate::views::compra::*;
use crate::State;
use serde_qs::Config;
use sqlx::PgPool;
use tide::Request;

/// formulario para ingresar nuevo registro
pub async fn ctrl_new(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let errores = String::new();
    let res = view_new(errores).await.unwrap();
    Ok(res)
}

/// api para crear un registro con los datos enviados
/// todo: hacer que si hay error muestre el contenido que lo provocó (view edit)
/// todo: debe mostrar el url correcto (muestra erprus/compras no erprus/compras/new)
/// todo: debe implementar una politica de manejo de errores (ver from zero 2 prod rust)
pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    // descartado temporalmente hasta saber como interpretar los corchetes codificados
    //let parametros: CompraForm = req.body_form().await?;

    // en lugar de la sentencia anterior
    // lee el formulario como string
    let parametros_str = req.body_string().await?;
    // ajusta el conversor a modo no estricto para que interprete los corchetes codificados
    let qs_non_strict = Config::new(5, false);
    // llena la structura con los parametros del string
    let parametros: CompraForm = qs_non_strict.deserialize_str(&parametros_str).unwrap();
    // fin en lugar de la sentencia anterior

    // construye la cabecera de la compra a partir de la estructura de parametros
    let compra = Compra::from_parametros_form(&parametros);

    // construye el detalle de la compra a partir de la estructura de parametros
    let mut comprados: Vec<Comprado> = Vec::new();
    for i in 0..parametros.producto_id.len() {
        comprados.push(Comprado::from_parametros_form(&parametros, i));
    }

    // crea la compra
    let res = match hndl_create(&db_pool, compra, comprados).await {
        Ok(row) => ctrl_new(req).await,
        Err(err) => view_edit(err.to_string()).await,
    };

    Ok(res.unwrap())
}
