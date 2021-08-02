use crate::models::empleado::Empleado;
use sqlx::{query, query_as, PgPool};
use tide::Error;

pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Empleado>> {
    let rows = sqlx::query_as!(
        Empleado,
        r#"
SELECT id, desde, hasta, activo, usuario_id, grupo_id, sucursal_id, created_at, updated_at
FROM empleados
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Empleado>> {
    let row = sqlx::query_as!(
        Empleado,
        r#"
SELECT id, desde, hasta, activo, usuario_id, grupo_id, sucursal_id, created_at, updated_at
FROM empleados
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Empleado) -> tide::Result<Empleado> {
    let row: Empleado = query_as!(
        Empleado,
        r#"
INSERT INTO  empleados (
    id, desde, hasta, activo, usuario_id, grupo_id, sucursal_id, created_at, updated_at
)
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )
RETURNING id, desde, hasta, activo, usuario_id, grupo_id, sucursal_id, created_at, updated_at
            "#,
        estructura.id,
        estructura.desde,
        estructura.hasta,
        estructura.activo,
        estructura.usuario_id,
        estructura.grupo_id,
        estructura.sucursal_id,
        estructura.created_at,
        estructura.updated_at
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(
    db_pool: &PgPool,
    estructura: &Empleado,
    id: i64,
) -> tide::Result<Option<Empleado>> {
    let row = query_as!(
        Empleado,
        r#"
UPDATE empleados
SET desde=$2,hasta=$3,activo=$4,usuario_id=$5,grupo_id=$6,sucursal_id=$7,created_at=$8,updated_at=$9
WHERE id = $1
RETURNING id, desde, hasta, activo, usuario_id, grupo_id, sucursal_id, created_at, updated_at
        "#,
        id,
        estructura.desde,
        estructura.hasta,
        estructura.activo,
        estructura.usuario_id,
        estructura.grupo_id,
        estructura.sucursal_id,
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
DELETE FROM empleados
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

// lista id y nombre para selects
pub async fn hndl_list_nombres(db_pool: &PgPool) -> Vec<(i64, String)>
//) -> std::result::Result<std::result::Result<std::vec::Vec<sqlx::postgres::PgRow>, sqlx::Error>, _>
{
    let sucursal_id: i64 = 1;
    let empleados: Vec<(i64, String)> = sqlx::query_as(
        r#"
SELECT e.id, u.nombre
FROM empleados e 
INNER JOIN usuarios u ON e.usuario_id = u.id
WHERE e.sucursal_id = $1 and e.activo 
    "#,
    )
    .bind(sucursal_id)
    .fetch_all(db_pool)
    .await
    .unwrap();
    empleados
    //Ok(empleados)
}
