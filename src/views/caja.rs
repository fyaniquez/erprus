//use crate::handlers::hndl_hallar;
use crate::handlers::caja::hndl_hallar;
use crate::models::caja::Caja;
use crate::views::layout::page;
use maud::html;
use sqlx::PgPool;
use tide::http::mime;
use tide::{Body, Request, Response};

pub async fn view_listar(title: &str, rows: Vec<Caja>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page(
        title,
        html! {
            table {
                tr {
                    th { "Id."}
                    th { "Descripción"}
                    th { "Desde"}
                    th { "Hasta"}
                    th { "Activa"}
                    th colspan="3";
                }
                @for row in i {
                    tr {
                        td { (row.id.to_string())}
                        td { (row.descripcion)}
                        td { (row.desde.to_string())}
                        td { (row.hasta.to_string())}
                        td { (row.activa)}
                        td {
                            a href={"/erprus/caja/" (row.id.to_string()) "/delete"} {
                                "Borrar"
                            }
                        }
                        td {
                            a href={"/erprus/caja/" (row.id.to_string()) "/editar"} {
                                "Editar"
                            }
                        }
                        td {
                            a href={"/erprus/caja/" (row.id.to_string()) "/ver"} {
                                "Ver"
                            }
                        }
                    }
                }
            }
        },
    );
    let m = markup.into_string();
    let b = Body::from_string(m);
    res.set_body(b);
    Ok(res)
}
pub async fn view_ver(req: Request<crate::State>, row: Caja) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page("Mostrar Caja", ver(row));
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_nuevo(req: Request<crate::State>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "Nueva caja",
        formulario("post", String::from("/erprus/caja/crear"), Caja::vacio()),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_editar(req: Request<crate::State>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let db_pool: PgPool = req.state().db_pool.clone();
    let id: i64 = (req.param("id").unwrap_or("0")).parse::<i64>().unwrap_or(0);
    let caja: Caja = hndl_hallar(&db_pool, id).await.unwrap().unwrap();
    let markup = page(
        "Editar caja",
        formulario("post", format!("/erprus/caja/{}", id), caja),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(metodo: &str, accion: String, row: Caja) -> maud::Markup {
    //form action="/erprus/caja/crear" method="post" enctype="application/json" {
    //form action="/erprus/caja/crear" method="post" enctype="multipart/form-data" {
    html!(
        //form action="/erprus/caja/crear" method="post" enctype="application/x-www-form-urlencoded" {
        form action=(accion) method=(metodo) enctype="application/x-www-form-urlencoded" {
            application/x-www-form-urlencoded
            label for="descripcion" {"Descripción"};
            input type="text" name="descripcion" value=(row.descripcion);
            label for="activa" {"Activa"};
            input type="text" name="activa" value=(row.activa);
            label for="desde" {"Desde"};
            input type="text" name="desde" value=(row.desde);
            label for="hasta" {"Hasta"};
            input type="text" name="hasta" value=(row.hasta);
            button type="submit" {"Grabar"};
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Caja) -> maud::Markup {
    html!(
        p {"Id: " (row.id)};
        p {"Descripción: " (row.descripcion)};
        p {"Activa: " (row.activa)};
        p {"Desde: " (row.desde.to_string())};
        p {"Hasta: " (row.hasta.to_string())};
        p {"Creación: " (row.created_at.to_string())};
        p {"Modificación: " (row.updated_at.to_string())};
    )
}
