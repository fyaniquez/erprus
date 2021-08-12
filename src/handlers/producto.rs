use crate::models::producto::Producto;
use sqlx::{query, query_as, PgPool};
use tide::Error;

pub async fn hndl_list_form(
    db_pool: &PgPool,
    filtro: &str,
    longitud: &str,
    pagina: i64,
) -> tide::Result<Vec<Producto>> {
    let mut longitud_str = String::new();
    let long = get_longitud(longitud);
    if longitud != "todos" {
        longitud_str += &format!("LIMIT {} OFFSET {}", longitud, (pagina - 1) * long);
    };
    let mut filtro_str = String::from("WHERE activo");
    if filtro != "" {
        filtro_str += &format!(" AND nombre ILIKE '%{}%'", filtro);
    }
    let query: &str = &format!(
        r#"
    SELECT id, nombre, descripcion, activo, created_at, updated_at, categoria_id, marca_id, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
    FROM productos
    {}
    ORDER BY nombre
    {}
            "#,
        filtro_str, longitud_str
    );
    let rows: Vec<Producto> = sqlx::query_as(query)
        .fetch_all(db_pool)
        .await
        .map_err(|e| Error::new(409, e))?;
    Ok(rows)
}

pub async fn hndl_list_json(
    db_pool: &PgPool,
    filtro: &str,
    longitud: &str,
    pagina: i64,
) -> tide::Result<String> {
    let mut longitud_str = String::new();
    let long = get_longitud(longitud);
    if longitud != "todos" {
        longitud_str += &format!("LIMIT {} OFFSET {}", longitud, (pagina - 1) * long);
    };
    let mut filtro_str = String::from("WHERE activo");
    if filtro != "" {
        filtro_str += &format!(" AND nombre ILIKE '%{}%'", filtro);
    }
    let query: &str = &format!(
        r#"
SELECT cast(json_agg(filas) as text)
FROM (
    SELECT id, nombre, descripcion, activo, created_at, updated_at, categoria_id, marca_id, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
    FROM productos
    {}
    ORDER BY nombre
    {}
) filas
        "#,
        filtro_str, longitud_str
    );
    let rows: (String,) = sqlx::query_as(query).fetch_one(db_pool).await?;
    Ok(rows.0)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get_form(db_pool: &PgPool, id: i64) -> tide::Result<Option<Producto>> {
    let row = sqlx::query_as!(
        Producto,
        r#"
SELECT id, nombre, descripcion, activo, categoria_id, marca_id, created_at, updated_at, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
FROM productos
WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}
/// Obtener un registros de acuerdo a su id
pub async fn hndl_get_json(db_pool: &PgPool, id: i64) -> tide::Result<String> {
    let row: (String,) = sqlx::query_as(
        r#"
SELECT cast(json_agg(filas) as text)
FROM (
SELECT id, nombre, descripcion, activo, categoria_id, marca_id, created_at, updated_at, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
FROM productos
WHERE id = $1
) filas
        "#,
    )
    .bind(id)
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}
/// Crear un registro nuevo
pub async fn hndl_create(db_pool: &PgPool, estructura: Producto) -> tide::Result<Producto> {
    let row: Producto = query_as!(
            Producto,
            r#"
INSERT INTO productos (
    nombre, descripcion, activo, categoria_id, marca_id, created_at, updated_at, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
)
VALUES ( $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13 )
RETURNING id, nombre, descripcion, activo, categoria_id, marca_id, created_at, updated_at, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
            "#,
estructura. nombre, estructura. descripcion, estructura. activo, estructura. categoria_id, estructura. marca_id, estructura. created_at, estructura. updated_at, estructura. unidad_id, estructura. barras, estructura. contenido, estructura. caracteristicas, estructura. fraccionable, estructura. cantidad,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Actualizar un registro existente
pub async fn hndl_update(
    db_pool: &PgPool,
    estructura: &Producto,
    id: i64,
) -> tide::Result<Option<Producto>> {
    let row = query_as!(
        Producto,
        r#"
UPDATE productos 
SET nombre=$2,descripcion=$3,activo=$4,categoria_id=$5,marca_id=$6,created_at=$7,updated_at=$8,unidad_id=$9,barras=$10,contenido=$11,caracteristicas=$12,fraccionable=$13,cantidad=$14
WHERE id = $1
RETURNING id, nombre, descripcion, activo, categoria_id, marca_id, created_at, updated_at, unidad_id, barras, contenido, caracteristicas, fraccionable, cantidad
        "#,
        id, estructura.nombre, estructura.descripcion, estructura.activo, estructura.categoria_id, estructura.marca_id, estructura.created_at, estructura.updated_at, estructura.unidad_id, estructura.barras, estructura.contenido, estructura.caracteristicas, estructura.fraccionable, estructura.cantidad,
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
DELETE FROM productos
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
/// Obtener el total de registros
pub async fn hndl_count(db_pool: &PgPool, filtro: &str) -> tide::Result<i64> {
    let mut filtro_str = String::from("WHERE activo");
    if filtro != "" {
        filtro_str += &format!(" AND nombre ILIKE '%{}%'", filtro);
    }
    let query: &str = &format!(
        r#"
SELECT count(id)
FROM productos
{}
        "#,
        filtro_str
    );
    let row: (i64,) = sqlx::query_as(query).fetch_one(db_pool).await?;
    Ok(row.0)
}

fn get_longitud(longitud: &str) -> i64 {
    match longitud {
        "todos" => 0,
        _ => longitud.parse::<i64>().unwrap(),
    }
}
