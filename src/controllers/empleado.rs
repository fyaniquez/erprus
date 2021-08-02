use crate::handlers::empleado::*;
use crate::models::empleado::Empleado;
use crate::views::empleado::*;
use crate::State;
use sqlx::PgPool;
use tide::Request;

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
    let res = view_new(errores).await.unwrap();
    Ok(res)
}

pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let parametros: Empleado = req.body_form().await?;
    let res = match hndl_create(&db_pool, parametros).await {
        Ok(row) => view_show(row).await.unwrap(),
        Err(err) => view_new(err.to_string()).await.unwrap(),
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
            view_edit(&row, id, errores).await.unwrap()
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

    let mut actualizado: Empleado = req.body_form().await?;
    let row = hndl_get(&db_pool, id).await?;
    let res = match row {
        None => return ctrl_list(req).await,
        Some(row) => {
            actualizado.id = row.id;
            actualizado.desde = row.desde;
            actualizado.hasta = row.hasta;
            actualizado.activo = row.activo;
            actualizado.usuario_id = row.usuario_id;
            actualizado.grupo_id = row.grupo_id;
            actualizado.sucursal_id = row.sucursal_id;
            actualizado.created_at = row.created_at;
            actualizado.updated_at = row.updated_at;

            let resp = match hndl_update(&db_pool, &actualizado, id).await {
                Ok(reg) => match reg {
                    None => ctrl_list(req).await,
                    Some(reg) => view_show(reg).await,
                },
                Err(err) => {
                    errores += &err.to_string();
                    let rows = crate::handlers::empleado::hndl_list(&db_pool)
                        .await
                        .unwrap();
                    view_edit(&actualizado, id, errores).await
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
    Ok(res)
}
