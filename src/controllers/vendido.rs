/// controller vendido
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.883475486 -04:00
///
use crate::handlers::vendido::*;
use crate::models::vendido::Vendido;
use crate::views::vendido::*;
use crate::State;
use sqlx::PgPool;
use tide::Request;

/*
pub async fn ctrl_list(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let rows = hndl_list(&db_pool).await?;
    let res = view_list(rows).await.unwrap();
    Ok(res)
}
*/
