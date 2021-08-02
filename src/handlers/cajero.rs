use crate::models::cajero::Cajero;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// Listar los registros existentes de acuerdo a los parametros
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Cajero>> {
    let rows = sqlx::query_as!(
        Cajero,
        r#"
SELECT e.id, e.apertura, e.cierre, e.inicial, e.terminal, e.ingresos, e.egresos, 
e.activo, e.admin_id, e.empleado_id, e.caja_id, e.created_at, e.updated_at
FROM cajeros e, cajas a
WHERE e.caja_id = a.id and a.sucursal_id = $1
        "#,
        1 // TODO: cambiar a la sucursal actual
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Listar los registros existentes de acuerdo a los parametros
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Cajero>> {
    let row = sqlx::query_as!(
        Cajero,
        r#"
SELECT id, apertura, cierre, inicial, terminal, ingresos, egresos, 
activo, admin_id, empleado_id, caja_id, created_at, updated_at
FROM cajeros
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
pub async fn hndl_create(db_pool: &PgPool, cajero: Cajero) -> tide::Result<Cajero> {
    let row: Cajero = query_as!(
        Cajero,
        r#"
INSERT INTO cajeros (
apertura, cierre, inicial, terminal, ingresos, egresos, 
admin_id, empleado_id, caja_id, created_at, updated_at
)
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 )
RETURNING id as "id!", apertura, cierre, inicial, terminal, ingresos, egresos, 
activo, admin_id, empleado_id, caja_id, created_at, updated_at
        "#,
        cajero.apertura,
        cajero.cierre,
        cajero.inicial,
        cajero.terminal,
        cajero.ingresos,
        cajero.egresos,
        cajero.admin_id,
        cajero.empleado_id,
        cajero.caja_id,
        cajero.created_at,
        cajero.updated_at
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(
    db_pool: &PgPool,
    cajero: &Cajero,
    id: i64,
) -> tide::Result<Option<Cajero>> {
    let row = query_as!(
        Cajero,
        r#"
UPDATE cajeros SET 
apertura=$2, cierre=$3, inicial=$4, terminal=$5, ingresos=$6, egresos=$7, 
admin_id=$8, empleado_id=$9, caja_id=$10, updated_at=$11
WHERE id = $1
RETURNING id as "id!", apertura, cierre, inicial, terminal, ingresos, egresos, 
activo, admin_id, empleado_id, caja_id, created_at, updated_at
        "#,
        id,
        cajero.apertura,
        cajero.cierre,
        cajero.inicial,
        cajero.terminal,
        cajero.ingresos,
        cajero.egresos,
        cajero.admin_id,
        cajero.empleado_id,
        cajero.caja_id,
        cajero.updated_at
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
        delete from cajeros
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
