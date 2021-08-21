/// view vendido
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:18.883649120 -04:00
///        
use maud::{html, Markup};

/// lista de registros en la tabla
pub fn formulario_detalle() -> Markup {
    html! {
    .detform {
        table#detform {
            tr {
                th { "id"}
                th { "producto"}
                th { "cantidad"}
                th { "precio"}
                th { "subtotal"}
                th { img src="/public/img/gear.png"; }
            }
            tr#detalle_0 {
                td { input type="text" name="producto_id[0]" id="producto_id_0"; }
                td { input type="text" id="nombre_0"; }
                td { input type="text" name="cantidad[0]" id="cantidad_0"; }
                td { input type="text" name="precio[0]" id="precio_0"; }
                td { input type="text" name="subtotal[0]" id="subtotal_0"; }
                td {
                    button type="button" class="btnverde" {"Cambiar"}
                    button type="button" class="btnrojo" {"Borrar"}
                }
            }
        }
        button#agregar class="btnverde" type="button" {"Agregar producto"}
        .modal#search {
            ul#tsearch;
        }
    }
        }
}
