/// view marca
/// autor: fyaniquez
/// fecha: 2021-07-12 19:22:46.428610390 -04:00
///        
use crate::models::marca::Marca;
use crate::views::layout::{page_form, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

/// lista de registros en la tabla
pub async fn view_list(rows: Vec<Marca>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page_form(
        "Lista marcas",
        "marca",
        "marca.js",
        "marca.css",
        html! {
                    table {
                        tr {
                            th { "id"}
        th { "nombre"}
        th { "fabrica_id"}
        th { "created_at"}
        th { "updated_at"}

                            th colspan="3";
                        }
                        @for row in i {
                            tr {
                                td { (row.id.to_string())}
        td { (row.nombre.to_string())}
        td { (row.fabrica_id.to_string())}
        td { (row.created_at.to_string())}
        td { (row.updated_at.to_string())}

                                td {
                                    a href={"/erprus/marca/" (row.id.to_string()) "/delete"} {
                                        "Borrar"
                                    }
                                }
                                td {
                                    a href={"/erprus/marca/" (row.id.to_string()) "/editar"} {
                                        "Editar"
                                    }
                                }
                                td {
                                    a href={"/erprus/marca/" (row.id.to_string())} {
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

/// registro individual
pub async fn view_show(row: Marca) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form("Mostrar Marca", "marca", "marca.js", "marca.css", ver(row));
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para ingresar nuevo registro
pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Nueva marca",
        "marca",
        "marca.js",
        "marca.css",
        formulario(String::from("/erprus/marca/crear"), &Marca::new(), errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para modificar registro
pub async fn view_edit(marca: &Marca, id: i64, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Editar marca",
        "marca",
        "marca.js",
        "marca.css",
        formulario(format!("/erprus/marca/{}", id), &marca, errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(accion: String, row: &Marca, errores: String) -> maud::Markup {
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
                label for="nombre" {"nombre"};
                input type="text" name="nombre" id="nombre" value=(row.nombre.to_string())
                    required placeholder="nombre";
            }
            .form-control {
                label for="fabrica_id" {"fabrica_id"};
                input type="text" name="fabrica_id" id="fabrica_id" value=(row.fabrica_id.to_string())
                    required placeholder="fabrica_id";
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
            a class="button" href="/erprus/marcas" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Marca) -> maud::Markup {
    html!(
            p { "id" (row.id.to_string()) }
    p { "nombre" (row.nombre.to_string()) }
    p { "fabrica_id" (row.fabrica_id.to_string()) }
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
