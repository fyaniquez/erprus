/// handler capitulo
/// autor: fyaniquez
/// fecha: 2021-07-12 18:54:46.497141369 -04:00
///
use crate::models::capitulo::Capitulo;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Capitulo>> {
    let rows = sqlx::query_as!(
        Capitulo,
        r#"
SELECT id,nombre,created_at,updated_at,descripcion
FROM capitulos
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Capitulo>> {
    let row = sqlx::query_as!(
        Capitulo,
        r#"
SELECT id,nombre,created_at,updated_at,descripcion
FROM capitulos
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Capitulo) -> tide::Result<Capitulo> {
    let row: Capitulo = query_as!(
        Capitulo,
        r#"
INSERT INTO capitulos (
    nombre,created_at,updated_at,descripcion
)
VALUES ( $1,$2,$3,$4 )
RETURNING id,nombre,created_at,updated_at,descripcion
            "#,
        estructura.nombre,
        estructura.created_at,
        estructura.updated_at,
        estructura.descripcion
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(
    db_pool: &PgPool,
    estructura: &Capitulo,
    id: i64,
) -> tide::Result<Option<Capitulo>> {
    let row = query_as!(
        Capitulo,
        r#"
UPDATE capitulos 
SET nombre=$2,created_at=$3,updated_at=$4,descripcion=$5
WHERE id = $1
RETURNING id,nombre,created_at,updated_at,descripcion
        "#,
        id,
        estructura.nombre,
        estructura.created_at,
        estructura.updated_at,
        estructura.descripcion
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
DELETE FROM capitulos
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
pub async fn hndl_list_nombres(db_pool: &PgPool) -> Vec<(i64, String)> {
    let rows: Vec<(i64, String)> = sqlx::query_as(
        r#"
SELECT id, nombre
FROM capitulos
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
    SELECT id, nombre FROM capitulos ORDER BY nombre
    ) filas
    "#,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
