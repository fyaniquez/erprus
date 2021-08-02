/// layout generico para las páginas web
/// autor: fyaniquez
/// fecha: 10/06/2021
///
/// Cabecera
/// acepta un título para la págiuna web
use maud::{html, Markup, DOCTYPE};

fn head(page_title: &str, script: &str, style: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" type="text/css" href={"/public/css/"(style)};
            script type="module" src={ "/public/js/"(script) } {}
            title { (page_title) }
        }
    }
}

/// logical header
fn header(opciones: Vec<(&str, &str)>) -> Markup {
    html! {
        header {
            topbar {
                ul {
                    li class="logo" {
                        img src="/public/img/logodoncoco2.png" alt="Almacén Don Coco";
                    }
                    @for opcion in opciones.into_iter() {
                        li class="menu" {
                            a href=(opcion.0) { (opcion.1) };
                        }
                    }
                }
                ul {
                    li class="menu" { a href="la.com" { "Inicia Sesión"} }
                    li class="menu" { a href="la.com" { "Regístrate"} }
                }
            }
        }
    }
}
fn subheader_form(page_title: &str, objeto: &str) -> Markup {
    html!(
        titlebar {
            h2 {(page_title)}
            ul {
                li class="title" { a href={"/erprus/" (objeto) "s/new"} { "Modificar"} }
                li class="title" { a href="la.com" { "eliminar"} }
            }
        }
    )
}
fn subheader_list(page_title: &str, objeto: &str, longitud: &str, filtro: &str) -> Markup {
    html!(
    titlebar {
        h2 {(page_title)}
        ul {
            li class="title" { a href={"/erprus/" (objeto) "s/new"} { "Agregar " (objeto)} }
            li class="title" { a href="la.com" { "Actualizar"} }
        }
    }
    searchbar {
        .longitud {
            label for="longitud" { "Mostrar " }
            select name="longitud" id="longitud" {
                option selected[longitud=="10"] {"10"}
                option selected[longitud=="25"] {"25"}
                option selected[longitud=="50"] {"50"}
                option selected[longitud=="todos"] {"todos"}
            }
            span {" registros"}
        }
        .filtro {
            label for="filtro" { "Buscar: "}
            input type="text" name="filtro" id="filtro" value=(filtro);
        }
    }
    )
}
/// A static content.
fn main_form(page_title: &str, objeto: &str, contenido: Markup) -> Markup {
    html! {
    main {
        (subheader_form(page_title, objeto))
        (contenido)
    }
    }
}
/// A static content.
fn main_list(
    page_title: &str,
    objeto: &str,
    nro_registros: i64,
    paginas: i64,
    pagina: i64,
    tomos: i64,
    longitud: &str,
    filtro: &str,
    contenido: Markup,
) -> Markup {
    html! {
    main {
        (subheader_list(page_title, objeto, longitud, filtro))
        (contenido)
        (subfooter_list(nro_registros, paginas, pagina, tomos))
    }
    }
}
/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            Made by Favio Yañiquez for
            a href="https://www.aymararu.com" target="_blank" {"Aymararu"}
        }
    }
}
fn subfooter_list(nro_registros: i64, paginas: i64, pagina: i64, tomos: i64) -> Markup {
    let tomo = (pagina - 1) / 10;
    let inferior = tomo * 10 + 1;
    let mut superior = inferior + 9;
    if superior > paginas {
        superior = paginas;
    }

    html! {
        pagebar#pagebar {
            .total_pages {
                (nro_registros) " registros encontrados"
            }
            .pages {
                span.page#primero { "<<" }
                span.page#previo { "<" }
                @for p in inferior..=superior {
                    @if p == pagina {
                        span.page#actual {(p)}
                    } @else {
                        span.page {(p)}
                    }
                }
                span.page#siguiente { ">" }
                span.page#ultimo data-index=(paginas) { ">>" }
            }
        }
    }
}
/// Layout para página genéria incluye `header` y `footer`.
///
/// El contenido va en el body mas el footer
pub fn page(
    tipo: &str,
    title: &str,
    objeto: &str,
    script: &str,
    style: &str,
    contenido: Markup,
) -> Markup {
    let opciones = vec![
        ("/erprus/productos", "Productos"),
        ("/erprus/ventas", "Ventas"),
        ("/erprus/compras", "Compras"),
    ];
    html! {
        (DOCTYPE)
        html {
            (head(title, script, style))
            body {
                (header(opciones))
                (main_form(title, objeto, contenido))
                (footer())
            }
        }
    }
}
pub fn page_list(
    title: &str,
    objeto: &str,
    script: &str,
    style: &str,
    nro_registros: i64,
    paginas: i64,
    pagina: i64,
    tomos: i64,
    longitud: &str,
    filtro: &str,
    contenido: Markup,
) -> Markup {
    let opciones = vec![
        ("/erprus/productos", "Productos"),
        ("/erprus/ventas", "Ventas"),
        ("/erprus/compras", "Compras"),
    ];
    html! {
        (DOCTYPE)
        html {
            (head(title, script, style))
            body {
                (header(opciones))
                (main_list(title, objeto, nro_registros, paginas, pagina, tomos, longitud, filtro, contenido))
                (footer())
            }
        }
    }
}
