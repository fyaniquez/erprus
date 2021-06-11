use crate::models::caja::Caja;
use crate::views::layout::page;
use maud::html;
use tide::http::mime;
use tide::{Body, Response};

pub async fn view_caja_list(title: &str, rows: Vec<Caja>) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let i = rows.into_iter();
    let markup = page(
        title,
        html! {
            table {
                @for row in i {
                    tr {
                        td { (row.id.to_string())}
                        td { (row.descripcion.unwrap())}
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
