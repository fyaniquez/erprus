/// view compra
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:06.537351881 -04:00
///        
use crate::models::compra::Compra;
use crate::views::layout::page_maestro_detalle;
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

/// formulario para ingresar nuevo registro
pub async fn view_new(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_maestro_detalle(
        "Nueva compra",
        "compra",
        "compra_form.js",
        "compra.css",
        formulario_maestro(String::from("/erprus/compras"), &Compra::new(), errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// formulario para modificar nuevo registro con errores
pub async fn view_edit(errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_maestro_detalle(
        "Nueva compra",
        "compra",
        "compra_form.js",
        "compra.css",
        formulario_maestro(String::from("/erprus/compras"), &Compra::new(), errores),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario_maestro(accion: String, row: &Compra, errores: String) -> maud::Markup {
    html!(
        @if errores.len() > 0 {
            p {
                "No se pudo grabar el registro por: "  (errores)
            }
        }
        form#mainform action=(accion) method="post" {
            .form-control {
                label for="total" {"total"};
                input type="text" name="total" id="total" value=(row.total.to_string())
                    required placeholder="total";
            }
            .form-control {
                label for="pago" {"pago"};
                input type="text" name="pago" id="pago" value=(row.pago.to_string())
                    required placeholder="pago";
            }
            .form-control {
                label for="factura" {"factura"};
                input type="text" name="factura" id="factura" value=(row.factura.to_string())
                    required placeholder="factura";
            }
            .form-control {
                label for="numero" {"numero"};
                input type="text" name="numero" id="numero" value=(row.numero)
                    required placeholder="numero";
            }
            .form-control {
                label for="observaciones" {"observaciones"};
                input type="text" name="observaciones" id="observaciones" value=(row.observaciones)
                    required placeholder="observaciones";
            }
            .form-control {
                label for="distribuidora_id" {"distribuidora_id"};
                input type="text" name="distribuidora_id" id="distribuidora_id" value=(row.distribuidora_id.to_string())
                    required placeholder="distribuidora_id";
            }
            (crate::views::vendido::formulario_detalle())
            button class="btnverde" type="submit" {"Grabar"}
            button class="btnrojo" type="button" {"Borrar"}
        }
    )
}
