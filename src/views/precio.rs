use crate::models::precio::ProductoPrecio;
use crate::views::layout::{page_form, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

pub async fn view_list_form(
    rows: Vec<ProductoPrecio>,
    nro_registros: i64,
    paginas: i64,
    pagina: i64,
    tomos: i64,
    longitud: &str,
    filtro: &str,
) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let registros = rows.into_iter();
    let markup = page_list(
        "Lista de precios",
        "precio",
        "precio_list.js",
        "precio_list.css",
        nro_registros,
        paginas,
        pagina,
        tomos,
        longitud,
        filtro,
        html! {
            table#reporte {
                colgroup {
                    col width="2rem";
                    col width="300rem";
                    col width="2rem";
                    col width="2rem";
                    col width="2rem";
                    col width="150rem";
                }
                tr {
                    th { "id"}
                    th { "nombre"}
                    th { "cantidad"}
                    th { "fraccionable"}
                    th { "precio"}
                    th;
                }
                @for row in registros {
                tr {
                    td { (row.id.to_string())}
                    td { (row.nombre)}
                    td { (row.cantidad.to_string())}
                    td { (row.fraccionable.to_string())}
                    td { (row.precio.to_string())}
                    td aria-rowindex=(row.id.to_string()) {
                        a class="btnverde" href={"/erprus/productos/" (row.id.to_string()) "/edit"} {"Cambiar"}
                        button type="button" class="btnrojo" {"Borrar"}
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

pub async fn view_list_json(row: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    res.set_body(Body::from_string(row));
    Ok(res)
}
pub async fn view_get_form(row: ProductoPrecio) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page_form(
        "Mostrar Precio",
        "precio",
        "precio.js",
        "precio.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}
pub async fn view_get_json(row: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    res.set_body(Body::from_string(row));
    Ok(res)
}
/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: ProductoPrecio) -> maud::Markup {
    html!(
    p { "id" (row.id.to_string()) }
    p { "nombre" (row.nombre.to_string()) }
    p { "fraccionable" (row.fraccionable.to_string()) }
    p { "cantidad" (row.cantidad.to_string()) }
    p { "precio" (row.precio.to_string()) }
    p { "costo" (row.costo.to_string()) }
    )
}
