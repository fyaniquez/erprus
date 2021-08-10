/// view vendido
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.883649120 -04:00
///        
use crate::models::vendido::Vendido;
use crate::views::layout::{page_form, page_list};
use maud::{html, Markup};
use tide::http::mime;
use tide::{Body, Response};

/// lista de registros en la tabla
pub fn formulario_detalle() -> Markup {
    html! {
        table {
            tr {
                th { "producto_id"}
                th { "cantidad"}
                th { "precio"}
                th { "subtotal"}
                th { img src="/public/img/gear.png"; }
            }
            tr {
                td { input type="text" name="producto_id[0]" id="producto_id_0"; }
                td { input type="text" name="cantidad[0]" id="cantidad_0"; }
                td { input type="text" name="precio[0]" id="precio_0"; }
                td { input type="text" name="subtotal[0]" id="subtotal_0"; }
                td {
                    button type="button" class="btnverde" {"Cambiar"}
                    button type="button" class="btnrojo" {"Borrar"}
                }
            }
        }
    }
}
