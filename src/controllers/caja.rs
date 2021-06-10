use crate::handlers::caja::list;
use crate::State;
use sqlx::PgPool;
use tide::{Body, Request, Response};

pub async fn controller_list(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let rows = list(&db_pool).await?;
    let mut res = Response::new(200);
    res.set_body(Body::from_json(&rows)?);
    Ok(res)
}
