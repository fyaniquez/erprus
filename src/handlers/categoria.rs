/// handler categoria
/// autor: fyaniquez
/// fecha: 2021-07-11 21:43:03.192234599 -04:00
///
use crate::models::categoria::Categoria;
use sqlx::{query, query_as, PgPool};
use tide::Error;

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Categoria>> {
    let rows = sqlx::query_as!(
        Categoria,
        r#"
SELECT id,nombre,capitulo_id,created_at,updated_at
FROM categorias
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Categoria>> {
    let row = sqlx::query_as!(
        Categoria,
        r#"
SELECT id,nombre,capitulo_id,created_at,updated_at
FROM categorias
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Categoria) -> tide::Result<Categoria> {
    let row: Categoria = query_as!(
        Categoria,
        r#"
INSERT INTO categorias (
    nombre,capitulo_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4 )
RETURNING id,nombre,capitulo_id,created_at,updated_at
            "#,
        estructura.nombre,
        estructura.capitulo_id,
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
    estructura: &Categoria,
    id: i64,
) -> tide::Result<Option<Categoria>> {
    let row = query_as!(
        Categoria,
        r#"
UPDATE categorias 
SET nombre=$2,capitulo_id=$3,created_at=$4,updated_at=$5
WHERE id = $1
RETURNING id,nombre,capitulo_id,created_at,updated_at
        "#,
        id,
        estructura.nombre,
        estructura.capitulo_id,
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
DELETE FROM categorias
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
// id
pub async fn hndl_list_nombres(db_pool: &PgPool, capitulo: i64) -> Vec<(i64, String)> {
    let rows: Vec<(i64, String)> = sqlx::query_as(
        r#"
SELECT id, nombre
FROM categorias
WHERE capitulo_id = $1
ORDER BY nombre
    "#,
    )
    .bind(capitulo)
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
    SELECT id, nombre FROM categorias
    WHERE capitulo_id = $1 ORDER BY nombre
) filas
    "#,
    )
    .bind(capitulo)
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}

// obtiene capitulo a partir de categoria
pub async fn hndl_get_capitulo(db_pool: &PgPool, categoria: i64) -> tide::Result<i64> {
    let row: (i64,) = sqlx::query_as(
        r#"
SELECT capitulo_id
FROM categorias
WHERE id = $1
    "#,
    )
    .bind(categoria)
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
