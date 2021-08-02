use crate::models::producto::Producto;
use crate::views::layout::{page, page_list};
use maud::html;
use tide::http::mime;
use tide::{Body, Response};
type TAuxiliares = (
    Vec<(i64, String)>,
    Vec<(i64, String)>,
    Vec<(i64, String)>,
    Vec<(i64, String, String)>,
);
pub async fn view_list(
    rows: Vec<Producto>,
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
        "Lista productos",
        "producto",
        "producto_list.js",
        "producto_list.css",
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
                    th { "activo"}
                    th;
                }
                @for row in registros {
                tr {
                    td { (row.id.to_string())}
                    td { (row.nombre)}
                    td { (row.cantidad.to_string())}
                    td { (row.fraccionable.to_string())}
                    td { (row.activo.to_string())}
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

pub async fn view_show(row: Producto) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Mostrar Producto",
        "producto",
        "producto.js",
        "producto.css",
        ver(row),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_new(ta: TAuxiliares, errores: String) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Nuevo producto",
        "producto",
        "producto.js",
        "producto.css",
        formulario(
            String::from("/erprus/productos"),
            &Producto::new(),
            ta,
            0,
            String::new(),
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

pub async fn view_edit(
    producto: &Producto,
    id: i64,
    ta: TAuxiliares,
    capitulo: i64,
    errores: String,
) -> tide::Result {
    let mut res = Response::builder(200).content_type(mime::HTML).build();
    let markup = page(
        "formulario",
        "Editar producto",
        "producto",
        "producto.js",
        "producto.css",
        formulario(
            format!("/erprus/productos/{}", id),
            &producto,
            ta,
            capitulo,
            errores,
        ),
    );
    res.set_body(Body::from_string(markup.into_string()));
    Ok(res)
}

/// Construir un formulario con los campos de la tabla
pub fn formulario(
    accion: String,
    row: &Producto,
    ta: TAuxiliares,
    capitulo_id: i64,
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
                label for="capitulo_id" {"Capitulo"};
                select#capitulo_id {
                    @for capitulo in ta.0.into_iter() {
                        option value=(capitulo.0)
                            selected[capitulo.0 == capitulo_id]
                            {(capitulo.1)}
                    }
                }
            }
            .form-control {
                label for="categoria_id" {"Categoria"};
                select#categoria_id name="categoria_id" {
                    @for categoria in ta.1.into_iter() {
                        option value=(categoria.0)
                            selected[categoria.0 == row.categoria_id]
                            {(categoria.1)}
                    }
                }
            }
            .form-control {
                label for="marca_id" {"Marca"};
                select#marca_id name="marca_id" {
                    @for marca in ta.2.into_iter() {
                        option value=(marca.0)
                            selected[marca.0 == row.marca_id]
                            {(marca.1)}
                    }
                }
            }
            .form-control {
                label for="unidad_id" {"Unidad"}
                select#unidad_id name="unidad_id" {
                    @for unidad in ta.3.into_iter() {
                        option value=(unidad.0) data-sigla=(unidad.2)
                            selected[unidad.0 == (row.unidad_id as i64)]
                            {(unidad.1)}
                    }
                }
                span#nombre_unidad;
            }
            .form-control {
                label for="contenido" {"Contenido"};
                input type="text" name="contenido" id="contenido" value=(row.contenido.to_string())
                    required placeholder="contenido";
            }
            .form-control {
                label for="caracteristicas" {"caracteristicas"};
                input type="text" name="caracteristicas" id="caracteristicas" value=(row.caracteristicas.to_string())
                    required placeholder="caracteristicas";
            }

            .form-control {
                label for="nombre" {"Nombre"}
                input#nombre type="text" name="nombre" value=(row.nombre) readonly;
            }
            .form-control {
                label for="descripcion" {"Descripcion"};
                input#descripcion type="text" name="descripcion"  value=(row.descripcion.to_string())
                    required placeholder="descripcion";
            }

            .form-control {
                label for="fraccionable" {"Fraccionable"};
                input type="checkbox" name="fraccionable" id="fraccionable" value=(row.fraccionable.to_string());
            }

            .form-control {
                label for="cantidad" {"Cantidad"};
                input type="text" name="cantidad" id="cantidad" value=(row.cantidad.to_string())
                    required placeholder="cantidad";
            }

            .form-control {
                label for="barras" {"Cód. barras"};
                input#barras type="text" name="barras" value=(row.barras.to_string())
                    required placeholder="barras";
            }
            button type="submit" {"Grabar"}
            a class="button" href="/erprus/productos" {"Cancelar"}
        }
    )
}

/// construir una ficha que muestre un registro en la tabla
pub fn ver(row: Producto) -> maud::Markup {
    html!(
    p { "id" (row.id.to_string()) }
    p { "nombre" (row.nombre.to_string()) }
    p { "descripcion" (row.descripcion.to_string()) }
    p { "activo" (row.activo.to_string()) }
    p { "categoria_id" (row.categoria_id.to_string()) }
    p { "marca_id" (row.marca_id.to_string()) }
    p { "created_at" (row.created_at.to_string()) }
    p { "updated_at" (row.updated_at.to_string()) }
    p { "unidad_id" (row.unidad_id.to_string()) }
    p { "barras" (row.barras.to_string()) }
    p { "contenido" (row.contenido.to_string()) }
    p { "caracteristicas" (row.caracteristicas.to_string()) }
    p { "fraccionable" (row.fraccionable.to_string()) }
    p { "cantidad" (row.cantidad.to_string()) }
    )
}
