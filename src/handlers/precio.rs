use crate::models::precio::ProductoPrecio;
use sqlx::{query_as, PgPool};
use tide::Error;

pub async fn hndl_list_form(
    db_pool: &PgPool,
    filtro: &str,
    longitud: &str,
    pagina: i64,
) -> tide::Result<Vec<ProductoPrecio>> {
    let parametros = get_parametros(filtro, longitud, pagina);
    let query: &str = &format!(
        r#"
    SELECT p.id, nombre, fraccionable, cantidad, precio, costo
    FROM productos p INNER JOIN precios r ON r.producto_id = p.id
    {}
    ORDER BY nombre
    {}
            "#,
        parametros.0, parametros.1
    );
    let rows: Vec<ProductoPrecio> = query_as(query)
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
    let parametros = get_parametros(filtro, longitud, pagina);
    let query: &str = &format!(
        r#"
SELECT cast(json_agg(filas) as text)
FROM (
    SELECT p.id, nombre, fraccionable, cantidad, precio, costo
    FROM productos p INNER JOIN precios r ON r.producto_id = p.id
    {}
    ORDER BY nombre
    {}
) filas
        "#,
        parametros.0, parametros.1
    );
    let rows: (String,) = query_as(query).fetch_one(db_pool).await?;
    Ok(rows.0)
}

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get_form(db_pool: &PgPool, id: i64) -> tide::Result<Option<ProductoPrecio>> {
    let row = query_as!(
        ProductoPrecio,
        r#"
SELECT p.id, nombre, fraccionable, cantidad, precio, costo
FROM productos p INNER JOIN precios r ON r.producto_id = p.id
WHERE activo AND p.id = $1 AND r.catalogo_id = $2
        "#,
        id,
        get_catalogo(),
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}

/// Obtener un registros de acuerdo a su id en formato json
pub async fn hndl_get_json(db_pool: &PgPool, id: i64) -> tide::Result<String> {
    let row: (String,) = query_as(
        r#"
SELECT cast(json_agg(filas) as text)
FROM (
    SELECT p.id, nombre, fraccionable, cantidad, precio, costo
    FROM productos p INNER JOIN precios r ON r.producto_id = p.id
    WHERE activo AND p.id = $1 AND r.catalogo_id = $2
) filas
        "#,
    )
    .bind(id)
    .bind(get_catalogo())
    .fetch_one(db_pool)
    .await?;
    Ok(row.0)
}

/// Obtener el total de registros
/// todo: hacer el catalogo una variable
pub async fn hndl_count(db_pool: &PgPool, filtro: &str) -> tide::Result<i64> {
    let mut filtro_str = format!("WHERE activo and catalogo_id = {}", get_catalogo());
    if filtro != "" {
        filtro_str += &format!(" AND nombre ILIKE '%{}%'", filtro);
    }
    let query: &str = &format!(
        r#"
SELECT count(p.id)
FROM productos p INNER JOIN precios r ON r.producto_id = p.id
{}
        "#,
        filtro_str
    );
    let row: (i64,) = query_as(query).fetch_one(db_pool).await?;
    Ok(row.0)
}

fn get_parametros(filtro: &str, longitud: &str, pagina: i64) -> (String, String) {
    let mut longitud_str = String::new();
    let long = match longitud {
        "todos" => 0,
        _ => longitud.parse::<i64>().unwrap(),
    };
    if longitud != "todos" {
        longitud_str += &format!("LIMIT {} OFFSET {}", longitud, (pagina - 1) * long);
    };
    let mut filtro_str = format!("WHERE p.activo AND catalogo_id = {}", get_catalogo());
    if filtro != "" {
        filtro_str += &format!(" AND p.nombre ILIKE '%{}%'", filtro);
    }
    (filtro_str, longitud_str)
}

fn get_catalogo() -> i64 {
    1
}
