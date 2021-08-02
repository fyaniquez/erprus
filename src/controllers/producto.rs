use crate::handlers::producto::*;
use crate::models::producto::Producto;
use crate::views::producto::*;
use crate::State;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tide::prelude::*;
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

// parametros para paginar la lista
#[derive(Deserialize)]
#[serde(default)]
struct ParametrosList {
    pub pagina: i64,
    pub filtro: String,
    pub longitud: String,
}
impl Default for ParametrosList {
    fn default() -> Self {
        Self {
            pagina: 1,
            filtro: String::new(),
            longitud: String::from("10"),
        }
    }
}
/// listar registros
pub async fn ctrl_list(mut req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let parametros: ParametrosList = req.query()?;
    let rows: Vec<Producto> = hndl_list(
        &db_pool,
        &parametros.filtro,
        &parametros.longitud,
        parametros.pagina,
    )
    .await?;
    let nro_registros = hndl_count(&db_pool, &parametros.filtro).await?;
    let paginas = get_paginas(&parametros.longitud, nro_registros);
    let tomos = get_tomos(paginas, 10);

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

/// obtiene un registro
pub async fn ctrl_get(req: Request<State>) -> tide::Result {
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let row = hndl_get(&db_pool, id).await?;
    let res = match row {
        None => ctrl_list(req).await.unwrap(),
        Some(row) => view_show(row).await.unwrap(),
    };
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
        Ok(row) => view_show(row).await.unwrap(),
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
    let row = hndl_get(&db_pool, id).await?;
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
    let row = hndl_get(&db_pool, id).await?;
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
                    Some(reg) => view_show(reg).await,
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

// calcula el numero de paginas
fn get_paginas(longitud: &str, nro_registros: i64) -> i64 {
    match longitud {
        "todos" => 1,
        _ => {
            let l = longitud.parse::<i64>().unwrap();
            if nro_registros % l > 0 {
                nro_registros / l + 1
            } else {
                nro_registros / l
            }
        }
    }
}

// fn calcula el numero de tomos = conjuntos de 10 paginas
fn get_tomos(paginas: i64, longitud_tomo: i64) -> i64 {
    if paginas % longitud_tomo > 0 {
        paginas / longitud_tomo + 1
    } else {
        paginas / longitud_tomo
    }
}
