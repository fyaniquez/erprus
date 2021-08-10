use crate::models::cajero::date_null;
use crate::models::cajero::Cajero;
use crate::views::layout::{page_form, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

pub async fn view_list(rows: Vec<Cajero>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page_form(
        "Lista cajeros",
        "cajero",
        "cajero.js",
        "cajero.css",
        html! {
            table {
                tr {
                    th { "Id."}
                    th { "Caja"}
                    th { "Cajero"}
                    th { "Administrador"}
                    th { "Apertura"}
                    th { "Cierre"}
                    th { "Inicial"}
                    th { "terminal"}
                    th { "Ingresos"}
                    th { "Egresos"}
                    th { "Activo"}
                    th colspan="3";
                }
                @for row in i {
                    tr {
                        td { (row.id.to_string())}
                        td { (row.caja_id.to_string())}
                        td { (row.empleado_id.to_string())}
                        td { (row.admin_id.to_string())}
                        td { (row.apertura.to_string())}
                        td { (date_null(row.cierre))}
                        td { (row.inicial.to_string())}
                        td { (row.terminal.to_string())}
                        td { (row.ingresos.to_string())}
                        td { (row.egresos.to_string())}
                        td { (row.activo)}
                        td {
                            a href={"/erprus/cajero/" (row.id.to_string()) "/delete"} {
                                "Borrar"
                            }
                        }
                        td {
                            a href={"/erprus/cajero/" (row.id.to_string()) "/editar"} {
                                "Editar"
                            }
                        }
                        td {
                            a href={"/erprus/cajero/" (row.id.to_string())} {
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
pub async fn view_show(row: Cajero) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Mostrar Cajero",
        "cajero",
        "cajero.js",
        "cajero.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_new(
    cajas: Vec<(i64, String)>,
    cajeros: Vec<(i64, String)>,
    errores: String,
) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Nueva cajero",
        "cajero",
        "cajero.js",
        "cajero.css",
        formulario(
            String::from("/erprus/cajero/crear"),
            &Cajero::vacio(),
            cajas,
            cajeros,
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_edit(
    cajero: &Cajero,
    id: i64,
    cajas: Vec<(i64, String)>,
    cajeros: Vec<(i64, String)>,
    errores: String,
) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Editar cajero",
        "cajero",
        "cajero.js",
        "cajero.css",
        formulario(
            format!("/erprus/cajero/{}", id),
            &cajero,
            cajas,
            cajeros,
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(
    accion: String,
    row: &Cajero,
    cajas: Vec<(i64, String)>,
    cajeros: Vec<(i64, String)>,
    errores: String,
) -> maud::Markup {
    html!(
        @if errores.len() > 0 {
            p {
                "No se pudo grabar el registro por: "  (errores)
            }
        }
        form#mainform action=(accion) method="post" {
            .form-control {
                label for="caja_id" {"Caja"};
                select name="caja_id" id="caja_id" {
                    @for caja in cajas.into_iter() {
                        option value=(caja.0) {
                            (caja.1)
                        }
                    }
                }
            }
            .form-control {
                label for="empleado_id" {"Cajero"};
                select name="empleado_id" id="empleado_id" {
                    @for cajero in cajeros.into_iter() {
                        option value=(cajero.0) {
                            (cajero.1)
                        }
                    }
                }
            }
            .form-control {
                label for="apertura" {"F.Apertura"};
                input type="datetime-local" name="apertura" id="apertura" value=(row.apertura.to_string())
                    required placeholder="DD-MM-YYYY HH:MM";
            }
            .form-control {
                label for="inicial" {"Saldo inicial"};
                input type="number" name="inicial" id="inicial" value=(row.inicial.to_string())
                    step="any" required placeholder="99999.99";
            }
            button type="submit" {"Grabar"}
            a class="button" href="/erprus/cajeros" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Cajero) -> maud::Markup {
    html!(
        p { "Id." (row.id.to_string())}
        p { "Caja" (row.caja_id.to_string())}
        p { "Cajero" (row.empleado_id.to_string())}
        p { "Administrador" (row.admin_id.to_string())}
        p { "Apertura" (row.apertura.to_string())}
        p { "Cierre" (
            match row.cierre {
                Some(f) => f.to_string(),
                None => "-.-".to_string(),
        })}
        p { "Inicial" (row.inicial.to_string())}
        p { "terminal" (
            match row.cierre {
                Some(f) => f.to_string(),
                None => "-.-".to_string(),
        })}
        p { "Ingresos" (row.ingresos.to_string())}
        p { "Egresos" (row.egresos.to_string())}
        p { "Activo" (row.activo)}
    )
}
