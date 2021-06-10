use crate::models::caja::Caja;
use sqlx::{query, query_as, PgPool};
use tide::{prelude, Error};

pub async fn list(db_pool: &PgPool) -> tide::Result<Vec<Caja>> {
    let rows = sqlx::query_as!(
            Caja,
            "Select id, descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at from cajas"
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}
