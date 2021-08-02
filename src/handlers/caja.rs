use crate::models::caja::Caja;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// Listar los registros existentes de acuerdo a los parametros
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Caja>> {
    let rows = sqlx::query_as!(
        Caja,
        r#"
SELECT id, descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at, numero 
FROM cajas
WHERE sucursal_id = $1
ORDER BY descripcion
        "#,
        1 // TODO: cambiar a la sucursal actual
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Listar los registros existentes de acuerdo a los parametros
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Caja>> {
    let row = sqlx::query_as!(
        Caja,
        r#"
SELECT id, descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at, numero 
FROM cajas
WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Crear un registro nuevo
pub async fn hndl_create(db_pool: &PgPool, caja: Caja) -> tide::Result<Caja> {
    let row: Caja = query_as!(
        Caja,
        r#"
INSERT INTO cajas (descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at, numero)
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
RETURNING id as "id!", descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at, numero
        "#,
        caja.descripcion,
        caja.activa,
        caja.sucursal_id,
        caja.desde,
        caja.hasta,
        caja.created_at,
        caja.updated_at,
        caja.numero
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(db_pool: &PgPool, caja: &Caja, id: i64) -> tide::Result<Option<Caja>> {
    let row = query_as!(
        Caja,
        r#"
UPDATE cajas SET descripcion=$2, numero=$3, desde=$4, hasta=$5, updated_at=$6
WHERE id = $1
RETURNING id, descripcion, activa, sucursal_id, desde, hasta, created_at, updated_at, numero
        "#,
        id,
        caja.descripcion,
        caja.numero,
        caja.desde,
        caja.hasta,
        caja.updated_at
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(row)
}

/// Eliminar registro
pub async fn hndl_delete(db_pool: &PgPool, id: i64) -> tide::Result<Option<()>> {
    let row = query!(
        r#"
        delete from cajas
        WHERE id = $1
        returning id
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    let r = match row {
        None => None,
        Some(_) => Some(()),
    };

    Ok(r)
}

// lista id y nombre para selects
pub async fn hndl_list_nombres(db_pool: &PgPool) -> Vec<(i64, String)>
//) -> std::result::Result<std::result::Result<std::vec::Vec<sqlx::postgres::PgRow>, sqlx::Error>, _>
{
    let sucursal_id: i64 = 1;
    sqlx::query_as(
        "
SELECT id, descripcion FROM cajas 
WHERE sucursal_id = $1 and activa",
    )
    .bind(sucursal_id)
    .fetch_all(db_pool)
    .await
    .unwrap()
}
