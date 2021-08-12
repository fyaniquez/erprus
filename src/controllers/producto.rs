use crate::handlers::producto::*;
use crate::models::parametros_list::ParametrosList;
use crate::models::producto::Producto;
use crate::views::producto::*;
use crate::State;
use chrono::prelude::*;
use http_types::mime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tide::Request;

type TAuxiliares = (
    Vec<(i64, String)>,
    Vec<(i64, String)>,
    Vec<(i64, String)>,
    Vec<(i64, String, String)>,
);

// parametros obtenidos del formulario
#[derive(Serialize, Deserialize, sqlx::Type)]
struct ParametrosForm {
    pub categoria_id: i64,
    pub marca_id: i64,
    pub unidad_id: i32,
    pub contenido: String,
    pub nombre: String,
    pub caracteristicas: String,
    pub descripcion: String,
    pub fraccionable: bool,
    pub cantidad: i32,
    pub barras: String,
}

/// listar registros
pub async fn ctrl_list(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let res;
    match req.content_type() {
        None => res = ctrl_list_form(req, &db_pool).await,
        Some(ct) => {
            if ct == mime::FORM {
                res = ctrl_list_form(req, &db_pool).await
            } else {
                res = ctrl_list_json(req, &db_pool).await
            }
        }
    };
    Ok(res.unwrap())
}
pub async fn ctrl_list_form(mut req: Request<State>, db_pool: &PgPool) -> tide::Result {
    let parametros: ParametrosList = req.query()?;
    let rows: Vec<Producto> = hndl_list_form(
        &db_pool,
        &parametros.filtro,
        &parametros.longitud,
        parametros.pagina,
    )
    .await?;
    let nro_registros = hndl_count(&db_pool, &parametros.filtro).await?;
    let paginas = parametros.get_paginas(nro_registros);
    let tomos = 0;

    let res = view_list(
        rows,
        nro_registros,
        paginas,
        parametros.pagina,
        tomos,
        &parametros.longitud,
        &parametros.filtro,
    )
    .await
    .unwrap();
    Ok(res)
}

/// listar registros
/// todo: hacer un procedimiento generico que permita elegir la salida html/json
pub async fn ctrl_list_json(mut req: Request<State>, db_pool: &PgPool) -> tide::Result {
    let parametros: ParametrosList = req.query()?;
    let rows = hndl_list_json(
        &db_pool,
        &parametros.filtro,
        &parametros.longitud,
        parametros.pagina,
    )
    .await?;
    let nro_registros = hndl_count(&db_pool, &parametros.filtro).await?;
    let paginas = parametros.get_paginas(nro_registros);
    let tomos = 0;

    let res = view_list_json(rows).await.unwrap();
    Ok(res)
}
/// obtiene un registro
pub async fn ctrl_get(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let res;
    match req.content_type() {
        None => res = ctrl_get_form(req, &db_pool, id).await,
        Some(ct) => {
            if ct == mime::FORM {
                res = ctrl_get_form(req, &db_pool, id).await
            } else {
                res = ctrl_get_json(req, &db_pool, id).await
            }
        }
    };
    Ok(res.unwrap())
}
/// obtiene un registro
pub async fn ctrl_get_form(req: Request<State>, db_pool: &PgPool, id: i64) -> tide::Result {
    let row = hndl_get_form(&db_pool, id).await?.unwrap();
    let res = view_get_form(row).await.unwrap();
    Ok(res)
}

/// obtiene un registro
pub async fn ctrl_get_json(req: Request<State>, db_pool: &PgPool, id: i64) -> tide::Result {
    let row = hndl_get_json(&db_pool, id).await?;
    let res = view_get_json(row).await.unwrap();
    Ok(res)
}

pub async fn ctrl_new(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let ta = tablas_auxiliares(&db_pool, 0, 0).await;
    let errores = String::new();
    let res = view_new(ta, errores).await.unwrap();

    Ok(res)
}

pub async fn ctrl_create(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let parametros: ParametrosForm = req.body_form().await?;
    let producto = Producto {
        id: 0,
        nombre: parametros.nombre,
        descripcion: parametros.descripcion,
        activo: true,
        categoria_id: parametros.categoria_id,
        marca_id: parametros.marca_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        unidad_id: parametros.unidad_id,
        barras: parametros.barras,
        contenido: parametros.contenido,
        caracteristicas: parametros.caracteristicas,
        fraccionable: parametros.fraccionable,
        cantidad: parametros.cantidad,
    };
    let res = match hndl_create(&db_pool, producto).await {
        Ok(row) => view_get_form(row).await.unwrap(),
        Err(err) => {
            let ta = tablas_auxiliares(&db_pool, 0, 0).await;
            view_new(ta, err.to_string()).await.unwrap()
        }
    };

    Ok(res)
}

pub async fn ctrl_edit(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_get_form(&db_pool, id).await?;
    let res = match row {
        None => ctrl_list(req).await.unwrap(),
        Some(row) => {
            let errores = String::new();
            let capitulo =
                crate::handlers::categoria::hndl_get_capitulo(&db_pool, row.categoria_id).await?;
            let ta = tablas_auxiliares(&db_pool, capitulo, row.categoria_id).await;
            view_edit(&row, id, ta, capitulo, errores).await.unwrap()
        }
    };
    Ok(res)
}

pub async fn ctrl_update(mut req: Request<State>) -> tide::Result {
    let mut errores = String::new();
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    if id == 0 {
        return ctrl_list(req).await;
    }

    let parametros: ParametrosForm = req.body_form().await?;
    let row = hndl_get_form(&db_pool, id).await?;
    let res = match row {
        None => return ctrl_list(req).await,
        Some(mut row) => {
            row.nombre = parametros.nombre;
            row.descripcion = parametros.descripcion;
            row.categoria_id = parametros.categoria_id;
            row.marca_id = parametros.marca_id;
            row.updated_at = Utc::now().naive_utc();
            row.unidad_id = parametros.unidad_id;
            row.barras = parametros.barras;
            row.contenido = parametros.contenido;
            row.caracteristicas = parametros.caracteristicas;
            row.fraccionable = parametros.fraccionable;
            row.cantidad = parametros.cantidad;

            let resp = match hndl_update(&db_pool, &row, id).await {
                Ok(reg) => match reg {
                    None => ctrl_list(req).await,
                    Some(reg) => view_get_form(reg).await,
                },
                Err(err) => {
                    errores += &err.to_string();
                    let capitulo =
                        crate::handlers::categoria::hndl_get_capitulo(&db_pool, row.categoria_id)
                            .await?;
                    let ta = tablas_auxiliares(&db_pool, capitulo, row.categoria_id).await;
                    view_edit(&row, id, ta, capitulo, errores).await
                }
            };
            resp
        }
    };
    res
}

pub async fn ctrl_delete(req: tide::Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);

    let _row = hndl_delete(&db_pool, id).await?;
    let res = ctrl_list(req).await.unwrap();
    Ok(res)
}

/// obtiene tablas auxiliares para poblar los selects
async fn tablas_auxiliares(db_pool: &PgPool, capitulo: i64, categoria: i64) -> TAuxiliares {
    let capitulo_id: i64;
    let mut categoria_id: i64;
    let capitulos = crate::handlers::capitulo::hndl_list_nombres(&db_pool).await;
    if capitulo < 1 {
        capitulo_id = capitulos[0].0;
    } else {
        capitulo_id = capitulo;
    }
    let categorias = crate::handlers::categoria::hndl_list_nombres(&db_pool, capitulo_id).await;
    if capitulo < 1 {
        categoria_id = categorias[0].0;
    } else {
        categoria_id = categoria;
    }
    let marcas = crate::handlers::marca::hndl_list_nombres(&db_pool, categoria_id).await;
    let unidades = crate::handlers::unidad::hndl_list_nombres(&db_pool).await;
    (capitulos, categorias, marcas, unidades)
}
