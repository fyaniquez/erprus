use crate::handlers::cajero::*;
use crate::models::cajero::Cajero;
use crate::views::cajero::*;
use crate::State;
use chrono::{prelude::*, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tide::Request;

// parametros obtenidos del formulario
#[derive(Serialize, Deserialize, sqlx::Type)]
struct Params {
    caja_id: i64,
    empleado_id: i64,
    apertura: NaiveDateTime,
    inicial: i32,
}
/// listar registros
pub async fn ctrl_list(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let rows = hndl_list(&db_pool).await?;
    let res = view_list(rows).await.unwrap();
    Ok(res)
}

/// obtiene un registro
pub async fn ctrl_get(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_get(&db_pool, id).await?;
    let res = match row {
        None => ctrl_list(req).await.unwrap(),
        Some(row) => view_show(row).await.unwrap(),
    };
    Ok(res)
}

pub async fn ctrl_new(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let errores = String::new();
    let cajas = crate::handlers::caja::hndl_list_nombres(&db_pool).await;
    let cajeros = crate::handlers::empleado::hndl_list_nombres(&db_pool).await;
    let res = view_new(cajas, cajeros, errores).await.unwrap();
    Ok(res)
}
pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let admin_id = 1; // todo colocar el usuario logueado
    let params: Params = req.body_form().await?;
    let cajero = Cajero {
        activo: true,
        admin_id: admin_id,
        created_at: Utc::now().naive_utc(),
        caja_id: params.caja_id,
        empleado_id: params.empleado_id,
        apertura: params.apertura,
        inicial: params.inicial,
        updated_at: Utc::now().naive_utc(),
        cierre: None,
        egresos: 0,
        terminal: 0,
        id: 0,
        ingresos: 0,
    };
    let res = match hndl_create(&db_pool, cajero).await {
        Ok(row) => view_show(row).await.unwrap(),
        Err(err) => {
            let cajas = crate::handlers::caja::hndl_list_nombres(&db_pool).await;
            let cajeros = crate::handlers::empleado::hndl_list_nombres(&db_pool).await;
            view_new(cajas, cajeros, err.to_string()).await.unwrap()
        }
    };

    Ok(res)
}

pub async fn ctrl_edit(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_get(&db_pool, id).await?;
    let res = match row {
        None => ctrl_list(req).await.unwrap(),
        Some(row) => {
            let errores = String::new();
            let cajas = crate::handlers::caja::hndl_list_nombres(&db_pool).await;
            let cajeros = crate::handlers::empleado::hndl_list_nombres(&db_pool).await;
            view_edit(&row, id, cajas, cajeros, errores).await.unwrap()
        }
    };
    Ok(res)
}

pub async fn ctrl_update(mut req: Request<State>) -> tide::Result {
    let mut errores = String::new();
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    if id == 0 {
        return ctrl_list(req).await;
    }
    let params: Params = req.body_form().await?;

    //valida_cajero
    // ... todo ...

    let row = hndl_get(&db_pool, id).await?;
    let res = match row {
        None => return ctrl_list(req).await,
        Some(row) => {
            let cajero = Cajero {
                activo: row.activo,
                admin_id: row.admin_id,
                created_at: row.created_at,
                caja_id: params.caja_id,
                empleado_id: params.empleado_id,
                apertura: params.apertura,
                inicial: params.inicial,
                updated_at: Utc::now().naive_utc(),
                cierre: None,
                egresos: 0,
                terminal: 0,
                id: 0,
                ingresos: 0,
            };
            let resp = match hndl_update(&db_pool, &cajero, id).await {
                Ok(reg) => match reg {
                    None => ctrl_list(req).await,
                    Some(reg) => view_show(reg).await,
                },
                Err(err) => {
                    errores += &err.to_string();
                    let cajas = crate::handlers::caja::hndl_list_nombres(&db_pool).await;
                    let cajeros = crate::handlers::empleado::hndl_list_nombres(&db_pool).await;
                    view_edit(&cajero, id, cajas, cajeros, errores).await
                }
            };
            resp
        }
    };
    res
}

pub async fn ctrl_delete(req: tide::Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);

    let _row = hndl_delete(&db_pool, id).await?;
    let res = ctrl_list(req).await.unwrap();
    //let res = match row {
    //    None => Response::new(404),
    //    Some(_) => Response::new(204),
    //};
    Ok(res)
}
