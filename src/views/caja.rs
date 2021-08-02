use crate::models::caja::Caja;
use crate::views::layout::page;
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

pub async fn view_list(rows: Vec<Caja>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page(
        "lista",
        "Lista cajas",
        "caja",
        "caja.js",
        "caja.css",
        html! {
            table {
                tr {
                    th { "Id."}
                    th { "Nro."}
                    th { "Descripción"}
                    th colspan="2" { "Horario atención" }
                    th { "Activa"}
                    th colspan="3";
                }
                @for row in i {
                    tr {
                        td { (row.id.to_string())}
                        td { (row.numero)}
                        td { (row.descripcion)}
                        td { (row.desde)}
                        td { (row.hasta)}
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
                            a href={"/erprus/caja/" (row.id.to_string())} {
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
pub async fn view_show(row: Caja) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Mostrar Caja",
        "caja",
        "caja.js",
        "caja.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Nueva caja",
        "caja",
        "caja.css",
        "caja.js",
        formulario(String::from("/erprus/caja/crear"), &Caja::vacio(), errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_edit(caja: &Caja, id: i64, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Editar caja",
        "caja",
        "caja.js",
        "caja.css",
        formulario(format!("/erprus/caja/{}", id), &caja, errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(accion: String, row: &Caja, errores: String) -> maud::Markup {
    html!(
        @if errores.len() > 0 {
            p {
                "No se pudo grabar el registro por: "  (errores)
            }
        }
        form#mainform action=(accion) method="post" {
            .form-control {
                label for="numero" {"Número"};
                input type="text" name="numero" id="numero" value=(row.numero)
                    required placeholder="Numero/Nombre caja";
            }
            .form-control {
                label for="descripcion" {"Descripción"};
                input type="text" name="descripcion" id="descripcion" value=(row.descripcion)
                    required placeholder="Ubicación, características";
            }
            .form-control {
                label for="desde" {"Desde"};
                input type="time" name="desde" id="desde" value=(row.desde)
                    required placeholder="HH:MM";
            }
            .form-control {
                label for="hasta" {"Hasta"};
                input type="time" name="hasta" id="hasta" value=(row.hasta)
                    required placeholder="HH:MM";
            }
            button type="submit" {"Grabar"}
            a class="button" href="/erprus/cajas" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Caja) -> maud::Markup {
    html!(
        p {"Id: " (row.id)};
        p {"Número: " (row.numero)};
        p {"Descripción: " (row.descripcion)};
        p {"Activa: " (row.activa)};
        p {"Desde: " (row.desde)};
        p {"Hasta: " (row.hasta)};
        p {"Creación: " (row.created_at.to_string())};
        p {"Modificación: " (row.updated_at.to_string())};
    )
}
