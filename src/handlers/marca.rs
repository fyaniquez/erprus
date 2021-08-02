/// handler marca
/// autor: fyaniquez
/// fecha: 2021-07-12 19:22:46.428261401 -04:00
///
use crate::models::marca::Marca;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Marca>> {
    let rows = sqlx::query_as!(
        Marca,
        r#"
SELECT id,nombre,fabrica_id,created_at,updated_at
FROM marcas
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Marca>> {
    let row = sqlx::query_as!(
        Marca,
        r#"
SELECT id,nombre,fabrica_id,created_at,updated_at
FROM marcas
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Marca) -> tide::Result<Marca> {
    let row: Marca = query_as!(
        Marca,
        r#"
INSERT INTO marcas (
    nombre,fabrica_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4 )
RETURNING id,nombre,fabrica_id,created_at,updated_at
            "#,
        estructura.nombre,
        estructura.fabrica_id,
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
    estructura: &Marca,
    id: i64,
) -> tide::Result<Option<Marca>> {
    let row = query_as!(
        Marca,
        r#"
UPDATE marcas 
SET nombre=$2,fabrica_id=$3,created_at=$4,updated_at=$5
WHERE id = $1
RETURNING id,nombre,fabrica_id,created_at,updated_at
        "#,
        id,
        estructura.nombre,
        estructura.fabrica_id,
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
DELETE FROM marcas
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

// lista id y nombre para selects en base a categoria
pub async fn hndl_list_nombres(db_pool: &PgPool, categoria: i64) -> Vec<(i64, String)> {
    let rows: Vec<(i64, String)> = sqlx::query_as(
        r#"
SELECT m.id, m.nombre
FROM marcas m
INNER JOIN categoriamarcas c
ON m.id = c.marca_id
WHERE c.categoria_id = $1
ORDER BY nombre
    "#,
    )
    .bind(categoria)
    .fetch_all(db_pool)
    .await
    .unwrap();
    rows
}

// lista id y nombre en json para selects
pub async fn hndl_list_nombres_json(db_pool: &PgPool, capitulo: i64) -> tide::Result<String> {
    let row: (String,) = sqlx::query_as(
        r#"
SELECT cast(json_agg(filas) as text) 
FROM ( 
    SELECT m.id, m.nombre FROM marcas m
    INNER JOIN categoriamarcas c ON m.id = c.marca_id
    WHERE c.categoria_id = $1 ORDER BY nombre
) filas
    "#,
    )
    .bind(capitulo)
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
