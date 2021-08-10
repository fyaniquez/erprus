use crate::models::empleado::Empleado;
use crate::views::layout::{page_form, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

pub async fn view_list(rows: Vec<Empleado>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page_form(
        "Lista empleados",
        "empleado",
        "empleado.js",
        "empleado.css",
        html! {
                    table {
                        tr {
                            th { "id"}
        th { " desde"}
        th { " hasta"}
        th { " activo"}
        th { " usuario_id"}
        th { " grupo_id"}
        th { " sucursal_id"}
        th { " created_at"}
        th { " updated_at"}

                            th colspan="3";
                        }
                        @for row in i {
                            tr {
                                td { (row.id.to_string())}
                                td { (row. desde.to_string())}
                                td { (row. hasta.to_string())}
                                td { (row. activo.to_string())}
                                td { (row. usuario_id.to_string())}
                                td { (row. grupo_id.to_string())}
                                td { (row. sucursal_id.to_string())}
                                td { (row. created_at.to_string())}
                                td { (row. updated_at.to_string())}
                                td {
                                    a href={"/erprus/empleado/" (row.id.to_string()) "/delete"} {
                                        "Borrar"
                                    }
                                }
                                td {
                                    a href={"/erprus/empleado/" (row.id.to_string()) "/editar"} {
                                        "Editar"
                                    }
                                }
                                td {
                                    a href={"/erprus/empleado/" (row.id.to_string())} {
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

pub async fn view_show(row: Empleado) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Mostrar Empleado",
        "empleado",
        "empleado.js",
        "empleado.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Nueva empleado",
        "empleado",
        "empleado.js",
        "empleado.css",
        formulario(
            String::from("/erprus/empleado/crear"),
            &Empleado::vacio(),
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_edit(empleado: &Empleado, id: i64, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Editar cajero",
        "empleado",
        "empleado.js",
        "empleado.css",
        formulario(format!("/erprus/empleado/{}", id), &empleado, errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(accion: String, row: &Empleado, errores: String) -> maud::Markup {
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
                label for="desde" {"desde"};
                input type="text" name="desde" id="desde" value=(row. desde.to_string())
                    required placeholder="desde";
            }
            .form-control {
                label for="hasta" {"hasta"};
                input type="text" name="hasta" id="hasta" value=(row. hasta.to_string())
                    required placeholder="hasta";
            }
            .form-control {
                label for="activo" {"activo"};
                input type="text" name="activo" id="activo" value=(row. activo.to_string())
                    required placeholder="activo";
            }
            .form-control {
                label for="usuario_id" {"usuario_id"};
                input type="text" name="usuario_id" id="usuario_id" value=(row. usuario_id.to_string())
                    required placeholder=" usuario_id";
            }
            .form-control {
                label for="grupo_id" {"grupo_id"};
                input type="text" name="grupo_id" id="grupo_id" value=(row. grupo_id.to_string())
                    required placeholder="grupo_id";
            }
            .form-control {
                label for="sucursal_id" {"sucursal_id"};
                input type="text" name="sucursal_id" id="sucursal_id" value=(row. sucursal_id.to_string())
                    required placeholder="sucursal_id";
            }
            .form-control {
                label for="created_at" {"created_at"};
                input type="text" name="created_at" id="created_at" value=(row. created_at.to_string())
                    required placeholder="created_at";
            }
            .form-control {
                label for=" updated_at" {" updated_at"};
                input type="text" name=" updated_at" id=" updated_at" value=(row. updated_at.to_string())
                    required placeholder=" updated_at";
            }
            button type="submit" {"Grabar"}
            a class="button" href="/erprus/empleados" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Empleado) -> maud::Markup {
    html!(
        p { "id" (row.id.to_string()) }
        p { " desde" (row.desde.to_string()) }
        p { " hasta" (row.hasta.to_string()) }
        p { " activo" (row.activo.to_string()) }
        p { " usuario_id" (row.usuario_id.to_string()) }
        p { " grupo_id" (row.grupo_id.to_string()) }
        p { " sucursal_id" (row.sucursal_id.to_string()) }
        p { " created_at" (row.created_at.to_string()) }
        p { " updated_at" (row.updated_at.to_string()) }
    )
}
