/// layout generico para las páginas web
/// autor: fyaniquez
/// fecha: 10/06/2021
///
/// Cabecera
/// acepta un título para la págiuna web
use maud::{html, Markup, DOCTYPE};

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        header {
            meta charset="utf-8";
            title { (page_title) }
        }
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            a href="rss.atom" { "RSS Feed" }
        }
    }
}

/// Layout para página genéria incluye `header` y `footer`.
///
/// El contenido va en el body mas el footer
pub fn page(title: &str, contenido: Markup) -> Markup {
    html! {
        (header(title))
        body {
            (contenido)
            (footer())
        }
    }
}
