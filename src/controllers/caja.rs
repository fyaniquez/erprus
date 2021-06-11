use crate::handlers::caja::hndl_caja_list;
use crate::views::caja::view_caja_list;
use crate::State;
use sqlx::PgPool;
use tide::{Request, Response};

pub async fn ctrl_caja_list(req: Request<State>) -> tide::Result {
    let title = String::from("lista cajas");
    let db_pool: PgPool = req.state().db_pool.clone();
    let rows = hndl_caja_list(&db_pool).await?;

    let res = view_caja_list(&title, rows).await.unwrap();
    Ok(res)
}
