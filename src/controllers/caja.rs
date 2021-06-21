use crate::handlers::caja::*;
use crate::models::caja::Caja;
use crate::views::caja::*;
use crate::State;
use sqlx::PgPool;
use tide::{Request, Response};

pub async fn ctrl_listar(req: Request<State>) -> tide::Result {
    let title = String::from("Lista cajas");
    let db_pool: PgPool = req.state().db_pool.clone();
    let rows = hndl_listar(&db_pool).await?;
    let res = view_listar(&title, rows).await.unwrap();
    Ok(res)
}

pub async fn ctrl_crear(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let caja: Caja = req.body_form().await?;
    let row = hndl_crear(&db_pool, caja).await?;
    let res = view_ver(req, row).await.unwrap();
    Ok(res)
}

pub async fn ctrl_modificar(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let caja: Caja = req.body_form().await?;
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_modificar(&db_pool, caja, id).await?;
    let res = view_ver(req, row).await.unwrap();
    Ok(res)
}

pub async fn ctrl_delete(req: tide::Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);

    let row = hndl_delete(&db_pool, id).await?;

    let res = match row {
        None => Response::new(404),
        Some(_) => Response::new(204),
    };
    Ok(res)
}
