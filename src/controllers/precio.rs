use crate::handlers::precio::{
    hndl_count, hndl_get_form, hndl_get_json, hndl_list_form, hndl_list_json,
};
use crate::models::parametros_list::ParametrosList;
use crate::models::precio::ProductoPrecio;
use crate::views::precio::{view_get_form, view_get_json, view_list_form, view_list_json};
use crate::State;
use http_types::mime;
use sqlx::PgPool;
use tide::Request;

/// listar registros
pub async fn ctrl_list(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let res;
    match req.content_type() {
        None => res = ctrl_list_form(req, &db_pool).await,
        Some(ct) => {
            if ct == mime::FORM {
                res = ctrl_list_form(req, &db_pool).await
            } else {
                res = ctrl_list_json(req, &db_pool).await
            }
        }
    };
    Ok(res.unwrap())
}
pub async fn ctrl_list_form(mut req: Request<State>, db_pool: &PgPool) -> tide::Result {
    let parametros: ParametrosList = req.query()?;
    let rows: Vec<ProductoPrecio> = hndl_list_form(
        &db_pool,
        &parametros.filtro,
        &parametros.longitud,
        parametros.pagina,
    )
    .await?;
    let nro_registros = hndl_count(&db_pool, &parametros.filtro).await?;
    let paginas = parametros.get_paginas(nro_registros);
    let tomos = 0; //get_tomos(paginas, 10);

    let res = view_list_form(
        rows,
        nro_registros,
        paginas,
        parametros.pagina,
        tomos,
        &parametros.longitud,
        &parametros.filtro,
    )
    .await
    .unwrap();
    Ok(res)
}

/// listar registros
/// todo: hacer un procedimiento generico que permita elegir la salida html/json
pub async fn ctrl_list_json(mut req: Request<State>, db_pool: &PgPool) -> tide::Result {
    let parametros: ParametrosList = req.query()?;
    let rows = hndl_list_json(
        &db_pool,
        &parametros.filtro,
        &parametros.longitud,
        parametros.pagina,
    )
    .await?;
    let nro_registros = hndl_count(&db_pool, &parametros.filtro).await?;
    let paginas = parametros.get_paginas(nro_registros);
    let tomos = 0; // get_tomos(paginas, 10);

    let res = view_list_json(rows).await.unwrap();
    Ok(res)
}
/// obtiene un registro
pub async fn ctrl_get(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let res;
    match req.content_type() {
        None => res = ctrl_get_form(req, &db_pool, id).await,
        Some(ct) => {
            if ct == mime::FORM {
                res = ctrl_get_form(req, &db_pool, id).await
            } else {
                res = ctrl_get_json(req, &db_pool, id).await
            }
        }
    };
    Ok(res.unwrap())
}
/// obtiene un registro
pub async fn ctrl_get_form(req: Request<State>, db_pool: &PgPool, id: i64) -> tide::Result {
    let row = hndl_get_form(&db_pool, id).await?.unwrap();
    let res = view_get_form(row).await.unwrap();
    Ok(res)
}

/// obtiene un registro
pub async fn ctrl_get_json(req: Request<State>, db_pool: &PgPool, id: i64) -> tide::Result {
    let row = hndl_get_json(&db_pool, id).await?;
    let res = view_get_json(row).await.unwrap();
    Ok(res)
}
