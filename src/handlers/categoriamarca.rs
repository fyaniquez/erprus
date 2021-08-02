
/// handler categoriamarca
/// autor: fyaniquez
/// fecha: 2021-07-17 21:33:19.166592908 -04:00
///
use crate::models::categoriamarca::Categoriamarca;
use sqlx::{query, query_as, PgPool};
use tide::Error;
        

/// consulta a la bd por registros de la tabla
pub async fn hndl_list(db_pool: &PgPool) -> tide::Result<Vec<Categoriamarca>> {
    let rows = sqlx::query_as!(
        Categoriamarca,
    r#"
SELECT id,categoria_id,marca_id,created_at,updated_at
FROM categoriamarcas
    "#)
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}


/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Categoriamarca>> {
    let row = sqlx::query_as!(
        Categoriamarca,
        r#"
SELECT id,categoria_id,marca_id,created_at,updated_at
FROM categoriamarcas
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
pub async fn hndl_create(db_pool: &PgPool, estructura: Categoriamarca) -> tide::Result<Categoriamarca> {
    let row: Categoriamarca = query_as!(
            Categoriamarca,
            r#"
INSERT INTO categoriamarcas (
    categoria_id,marca_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4 )
RETURNING id,categoria_id,marca_id,created_at,updated_at
            "#,
estructura.categoria_id, estructura.marca_id, estructura.created_at, estructura.updated_at
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    
    Ok(row)
}


/// Actualizar un registro existente
pub async fn hndl_update(db_pool: &PgPool, estructura: &Categoriamarca, id: i64) -> tide::Result<Option<Categoriamarca>> {
    let row = query_as!(
        Categoriamarca,
        r#"
UPDATE categoriamarcas 
SET categoria_id=$2,marca_id=$3,created_at=$4,updated_at=$5
WHERE id = $1
RETURNING id,categoria_id,marca_id,created_at,updated_at
        "#,
        id, estructura.categoria_id, estructura.marca_id, estructura.created_at, estructura.updated_at
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
DELETE FROM categoriamarcas
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

