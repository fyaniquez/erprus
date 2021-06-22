use crate::handlers::caja::*;
use crate::models::caja::Caja;
use crate::views::caja::*;
use crate::State;
use sqlx::PgPool;
use tide::{Request, Response};

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
        None => Response::new(404),
        Some(row) => view_show(req, row).await.unwrap(),
    };
    Ok(res)
}

pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let caja: Caja = req.body_form().await?;
    let row = hndl_create(&db_pool, caja).await?;
    let res = view_show(req, row).await.unwrap();
    Ok(res)
}

pub async fn ctrl_update(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let caja: Caja = req.body_form().await?;
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_update(&db_pool, caja, id).await?;
    let res = match row {
        None => Response::new(404),
        Some(row) => view_show(req, row).await.unwrap(),
    };
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
