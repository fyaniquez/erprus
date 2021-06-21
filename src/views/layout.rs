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
        head {
            meta charset="utf-8";
            title { (page_title) }
            link rel="stylesheet" type="text/css" href="/public/css/layout.css";
        }
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
        }
    }
}

/// un marco para la aplicación
fn marco(page_title: &str) -> Markup {
    html! {
        .cabecera {
            img src="/public/img/doncoco.png" alt="Almacén Don Coco";
            ul class="ingreso" {
                li {
                    a href="la.com" { "Inicia Sesión"}
                    a href="la.com" { "Regístrate"}
                }
            }
        }
        nav {
            ul {
                li {
                    a href="a.com" { "opcion 1" };
                    a href="b.com" { "opcion 2" };
                    a href="s.com" { "salir" };
                }
            }
        }
        h2 { (page_title) }
    }
}

/// Layout para página genéria incluye `header` y `footer`.
///
/// El contenido va en el body mas el footer
pub fn page(title: &str, contenido: Markup) -> Markup {
    html! {
        (header(title))
        body {
            (marco(title))
            (contenido)
            (footer())
        }
    }
}
