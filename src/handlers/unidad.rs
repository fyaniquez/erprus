/// handler unidad
/// autor: fyaniquez
/// fecha: 2021-07-18 21:31:53.588490062 -04:00
///
use crate::models::unidad::Unidad;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Unidad>> {
    let rows = sqlx::query_as!(
        Unidad,
        r#"
SELECT id,sigla,nombre,created_at,updated_at
FROM unidades
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Unidad>> {
    let row = sqlx::query_as!(
        Unidad,
        r#"
SELECT id,sigla,nombre,created_at,updated_at
FROM unidades
WHERE id = $1
ORDER BY nombre
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Crear un registro nuevo
pub async fn hndl_create(db_pool: &PgPool, estructura: Unidad) -> tide::Result<Unidad> {
    let row: Unidad = query_as!(
        Unidad,
        r#"
INSERT INTO unidades (
    sigla,nombre,created_at,updated_at
)
VALUES ( $1,$2,$3,$4 )
RETURNING id,sigla,nombre,created_at,updated_at
            "#,
        estructura.sigla,
        estructura.nombre,
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
    estructura: &Unidad,
    id: i64,
) -> tide::Result<Option<Unidad>> {
    let row = query_as!(
        Unidad,
        r#"
UPDATE unidades 
SET sigla=$2,nombre=$3,created_at=$4,updated_at=$5
WHERE id = $1
RETURNING id,sigla,nombre,created_at,updated_at
        "#,
        id,
        estructura.sigla,
        estructura.nombre,
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
DELETE FROM unidades
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
pub async fn hndl_list_nombres(db_pool: &PgPool) -> Vec<(i64, String, String)> {
    let rows: Vec<(i64, String, String)> = sqlx::query_as(
        r#"
SELECT id, sigla, nombre
FROM unidades
ORDER BY nombre
    "#,
    )
    .fetch_all(db_pool)
    .await
    .unwrap();
    rows
}

// lista id y nombre en json para selects
pub async fn hndl_list_nombres_json(db_pool: &PgPool) -> tide::Result<String> {
    let row: (String,) = sqlx::query_as(
        r#"
SELECT cast(json_agg(filas) as text) 
FROM ( 
    SELECT id, nombre FROM unidades ORDER BY nombre
    ) filas
        "#,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
