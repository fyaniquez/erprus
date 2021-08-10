/// handler vendido
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.883142546 -04:00
///
use crate::models::vendido::Vendido;
use sqlx::{query, query_as, PgPool, Postgres, Transaction};
use tide::Error;

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Vendido>> {
    let rows = sqlx::query_as!(
        Vendido,
        r#"
SELECT id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
FROM vendidos
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Vendido>> {
    let row = sqlx::query_as!(
        Vendido,
        r#"
SELECT id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
FROM vendidos
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Vendido) -> tide::Result<Vendido> {
    let row: Vendido = query_as!(
        Vendido,
        r#"
INSERT INTO vendidos (
    cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4,$5,$6,$7 )
RETURNING id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
            "#,
        estructura.cantidad,
        estructura.precio,
        estructura.subtotal,
        estructura.producto_id,
        estructura.venta_id,
        estructura.created_at,
        estructura.updated_at
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Crear un registro nuevo
pub async fn hndl_create_tran<'a>(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    estructura: &Vendido,
) -> tide::Result<Vendido> {
    let row: Vendido = query_as!(
        Vendido,
        r#"
INSERT INTO vendidos (
    cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4,$5,$6,$7 )
RETURNING id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
            "#,
        estructura.cantidad,
        estructura.precio,
        estructura.subtotal,
        estructura.producto_id,
        id,
        estructura.created_at,
        estructura.updated_at
    )
    .fetch_one(tx)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(
    db_pool: &PgPool,
    estructura: &Vendido,
    id: i64,
) -> tide::Result<Option<Vendido>> {
    let row = query_as!(
        Vendido,
        r#"
UPDATE vendidos 
SET cantidad=$2,precio=$3,subtotal=$4,producto_id=$5,venta_id=$6,created_at=$7,updated_at=$8
WHERE id = $1
RETURNING id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
        "#,
        id,
        estructura.cantidad,
        estructura.precio,
        estructura.subtotal,
        estructura.producto_id,
        estructura.venta_id,
        estructura.created_at,
        estructura.updated_at
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
DELETE FROM vendidos
WHERE id = $1
RETURNING id
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

// lista id y nombre en json para selects
pub async fn hndl_list_nombres_json(db_pool: &PgPool) -> tide::Result<String> {
    let row: (String,) = sqlx::query_as(
        r#"
SELECT cast(json_agg(filas) as text) 
FROM ( 
    SELECT id, nombre FROM vendidos
    ORDER BY nombre
) filas
    "#,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
