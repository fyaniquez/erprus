/// view unidad
/// autor: fyaniquez
/// fecha: 2021-07-18 21:31:53.608694802 -04:00
///        
use crate::models::unidad::Unidad;
use crate::views::layout::{page_form, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

/// lista de registros en la tabla
pub async fn view_list(rows: Vec<Unidad>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page_form(
        "Lista unidades",
        "unidad",
        "unidad.js",
        "unidad.css",
        html! {
            table {
                tr {
                    th { "id"}
                    th { "sigla"}
                    th { "nombre"}
                    th { "created_at"}
                    th { "updated_at"}
                    th colspan="3";
                }
                @for row in i {
                tr {
                    td { (row.id.to_string())}
                    td { (row.sigla)}
                    td { (row.nombre)}
                    td { (row.created_at.to_string())}
                    td { (row.updated_at.to_string())}
                    td { a href={"/erprus/unidad/" (row.id.to_string()) "/delete"} {"Borrar"}
                    }
                    td {a href={"/erprus/unidad/" (row.id.to_string()) "/editar"} {"Editar"}
                    }
                    td {a href={"/erprus/unidad/" (row.id.to_string())} {"Ver"}
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

/// registro individual
pub async fn view_show(row: Unidad) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Mostrar Unidad",
        "unidad",
        "unidad.js",
        "unidad.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para ingresar nuevo registro
pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Nueva unidad",
        "unidad",
        "unidad.js",
        "unidad.css",
        formulario(
            String::from("/erprus/unidad/crear"),
            &Unidad::new(),
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para modificar registro
pub async fn view_edit(unidad: &Unidad, id: i64, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Editar unidad",
        "unidad",
        "unidad.js",
        "unidad.css",
        formulario(format!("/erprus/unidad/{}", id), &unidad, errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(accion: String, row: &Unidad, errores: String) -> maud::Markup {
    html!(
        @if errores.len() > 0 {
            p {
                "No se pudo grabar el registro por: "  (errores)
            }
        }
        form#mainform action=(accion) method="post" {
            .form-control {
                label for="id" {"id"};
                input type="text" name="id" id="id" value=(row.id.to_string())
                    required placeholder="id";
            }
            .form-control {
                label for="sigla" {"sigla"};
                input type="text" name="sigla" id="sigla" value=(row.sigla)
                    required placeholder="sigla";
            }
            .form-control {
                label for="nombre" {"nombre"};
                input type="text" name="nombre" id="nombre" value=(row.nombre)
                    required placeholder="nombre";
            }
            .form-control {
                label for="created_at" {"created_at"};
                input type="text" name="created_at" id="created_at" value=(row.created_at.to_string())
                    required placeholder="created_at";
            }
            .form-control {
                label for="updated_at" {"updated_at"};
                input type="text" name="updated_at" id="updated_at" value=(row.updated_at.to_string())
                    required placeholder="updated_at";
            }
            button type="submit" {"Grabar"}
            a class="button" href="/erprus/unidads" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Unidad) -> maud::Markup {
    html!(
            p { "id" (row.id.to_string()) }
    p { "sigla" (row.sigla) }
    p { "nombre" (row.nombre) }
    p { "created_at" (row.created_at.to_string()) }
    p { "updated_at" (row.updated_at.to_string()) }
        )
}

/// lista de registros en la tabla en formato json
pub async fn view_list_nombres_json(row: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    res.set_body(Body::from_string(row));
    Ok(res)
}
