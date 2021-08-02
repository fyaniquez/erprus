/// view categoriamarca
/// autor: fyaniquez
/// fecha: 2021-07-17 21:33:19.175613990 -04:00
///        
use crate::models::categoriamarca::Categoriamarca;
use crate::views::layout::page;
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

/// lista de registros en la tabla
pub async fn view_list(rows: Vec<Categoriamarca>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page(
        "lista",
        "Lista categoriamarcas",
        "categoriamarca",
        "categoriamarca.js",
        "categoriamarca.css",
        html! {
                    table {
                        tr {
                            th { "id"}
        th { "categoria_id"}
        th { "marca_id"}
        th { "created_at"}
        th { "updated_at"}

                            th colspan="3";
                        }
                        @for row in i {
                            tr {
                                td { (row.id.to_string())}
        td { (row.categoria_id.to_string())}
        td { (row.marca_id.to_string())}
        td { (row.created_at.to_string())}
        td { (row.updated_at.to_string())}

                                td {
                                    a href={"/erprus/categoriamarca/" (row.id.to_string()) "/delete"} {
                                        "Borrar"
                                    }
                                }
                                td {
                                    a href={"/erprus/categoriamarca/" (row.id.to_string()) "/editar"} {
                                        "Editar"
                                    }
                                }
                                td {
                                    a href={"/erprus/categoriamarca/" (row.id.to_string())} {
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
pub async fn view_show(row: Categoriamarca) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Mostrar Categoriamarca",
        "categoriamarca",
        "categoriamarca.js",
        "categoriamarca.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para ingresar nuevo registro
pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Nueva categoriamarca",
        "categoriamarca",
        "categoriamarca.js",
        "categoriamarca.css",
        formulario(
            String::from("/erprus/categoriamarca/crear"),
            &Categoriamarca::vacio(),
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para modificar registro
pub async fn view_edit(categoriamarca: &Categoriamarca, id: i64, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Editar cajero",
        "categoriamarca",
        "categoriamarca.js",
        "categoriamarca.css",
        formulario(
            format!("/erprus/categoriamarca/{}", id),
            &categoriamarca,
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(accion: String, row: &Categoriamarca, errores: String) -> maud::Markup {
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
                label for="categoria_id" {"categoria_id"};
                input type="text" name="categoria_id" id="categoria_id" value=(row.categoria_id.to_string())
                    required placeholder="categoria_id";
            }
            .form-control {
                label for="marca_id" {"marca_id"};
                input type="text" name="marca_id" id="marca_id" value=(row.marca_id.to_string())
                    required placeholder="marca_id";
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
            a class="button" href="/erprus/categoriamarcas" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Categoriamarca) -> maud::Markup {
    html!(
            p { "id" (row.id.to_string()) }
    p { "categoria_id" (row.categoria_id.to_string()) }
    p { "marca_id" (row.marca_id.to_string()) }
    p { "created_at" (row.created_at.to_string()) }
    p { "updated_at" (row.updated_at.to_string()) }

        )
}
