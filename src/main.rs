use sqlx::{PgPool, Pool};
use tide::http::mime;
use tide::utils;
use tide::{
    //http::cookies::SameSite, prelude::*, Middleware, Next, Request, Response, Server, StatusCode,
    prelude::*,
    Response,
    Server,
};

mod controllers;
mod handlers;
mod models;
mod views;

use controllers::caja;
use controllers::cajero;
use controllers::capitulo;
use controllers::categoria;
use controllers::empleado;
use controllers::marca;
use controllers::precio;
use controllers::producto;
use controllers::unidad;
use controllers::venta;

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let db_pool = make_db_pool(&db_url).await;

    let app = server(db_pool).await;
    let mut listener = app
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("error al enlazar el puerto");

    for info in listener.info().iter() {
        println!("Servidor activado en {}", info);
    }
    listener.accept().await.unwrap();
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.unwrap()
}

async fn server(db_pool: PgPool) -> Server<State> {
    let state = State { db_pool };
    let mut app = tide::with_state(state);

    app.at("/public/js")
        .with(utils::After(|mut res: Response| async move {
            res.set_content_type(mime::JAVASCRIPT);
            Ok(res)
        }))
        .serve_dir("./public/js")
        .expect("Error en directorio estático");

    // statics
    app.at("/public")
        .serve_dir("./public/")
        .expect("Error en directorio estático");

    // api
    app.at("/erprus/cajas").post(caja::ctrl_create);
    app.at("/erprus/cajas/:id").post(caja::ctrl_update);
    app.at("/erprus/cajas/:id").delete(caja::ctrl_delete);
    app.at("/erprus/cajas").get(caja::ctrl_list);
    app.at("/erprus/cajas/:id").get(caja::ctrl_get);
    app.at("/erprus/cajas/new").get(caja::ctrl_new);
    app.at("/erprus/cajas/:id/edit").get(caja::ctrl_edit);

    app.at("/erprus/cajeros/crear").post(cajero::ctrl_create);
    app.at("/erprus/cajeros/:id").post(cajero::ctrl_update);
    app.at("/erprus/cajeros/:id/delete")
        .get(cajero::ctrl_delete);
    app.at("/erprus/cajeros").get(cajero::ctrl_list);
    app.at("/erprus/cajeros/:id").get(cajero::ctrl_get);
    app.at("/erprus/cajeros/nuevo").get(cajero::ctrl_new);
    app.at("/erprus/cajeros/:id/editar").get(cajero::ctrl_edit);

    app.at("/erprus/empleados/crear")
        .post(empleado::ctrl_create);
    app.at("/erprus/empleados/:id").post(empleado::ctrl_update);
    app.at("/erprus/empleados/:id/delete")
        .get(empleado::ctrl_delete);
    app.at("/erprus/empleados").get(empleado::ctrl_list);
    app.at("/erprus/empleados/:id").get(empleado::ctrl_get);
    app.at("/erprus/empleados/nuevo").get(empleado::ctrl_new);
    app.at("/erprus/empleados/:id/editar")
        .get(empleado::ctrl_edit);

    app.at("/erprus/productos").post(producto::ctrl_create);
    app.at("/erprus/productos/:id").post(producto::ctrl_update);
    app.at("/erprus/productos/:id")
        .delete(producto::ctrl_delete);
    app.at("/erprus/productos").get(producto::ctrl_list);
    app.at("/erprus/productos/:id").get(producto::ctrl_get);
    app.at("/erprus/productos/new").get(producto::ctrl_new);
    app.at("/erprus/productos/:id/edit")
        .get(producto::ctrl_edit);

    app.at("/erprus/precios").get(precio::ctrl_list);
    app.at("/erprus/precios/:id").get(precio::ctrl_get);

    app.at("/erprus/capitulos").post(capitulo::ctrl_create);
    app.at("/erprus/capitulos/:id").post(capitulo::ctrl_update);
    app.at("/erprus/capitulos/:id")
        .delete(capitulo::ctrl_delete);
    app.at("/erprus/capitulos").get(capitulo::ctrl_list);
    app.at("/erprus/capitulos/:id").get(capitulo::ctrl_get);
    app.at("/erprus/capitulos/new").get(capitulo::ctrl_new);
    app.at("/erprus/capitulos/:id/edit")
        .get(capitulo::ctrl_edit);
    app.at("/erprus/capitulos_nombres.json")
        .get(capitulo::ctrl_list_nombres_json);

    app.at("/erprus/categorias").post(categoria::ctrl_create);
    app.at("/erprus/categorias/:id")
        .post(categoria::ctrl_update);
    app.at("/erprus/categorias/:id")
        .delete(categoria::ctrl_delete);
    app.at("/erprus/categorias").get(categoria::ctrl_list);
    app.at("/erprus/categorias/:id").get(categoria::ctrl_get);
    app.at("/erprus/categorias/new").get(categoria::ctrl_new);
    app.at("/erprus/categorias/:id/edit")
        .get(categoria::ctrl_edit);
    app.at("/erprus/capitulos/:id/categorias_nombres.json")
        .get(categoria::ctrl_list_nombres_json);

    app.at("/erprus/categorias/:id/marcas_nombres.json")
        .get(marca::ctrl_list_nombres_json);

    app.at("/erprus/unidades_nombres.json")
        .get(unidad::ctrl_list_nombres_json);

    app.at("/erprus/ventas").post(venta::ctrl_create);
    //app.at("/erprus/ventas/:id").post(venta::ctrl_update);
    //app.at("/erprus/ventas/:id").delete(venta::ctrl_delete);
    //app.at("/erprus/ventas").get(venta::ctrl_list);
    //app.at("/erprus/ventas/:id").get(venta::ctrl_get);
    app.at("/erprus/ventas/new").get(venta::ctrl_new);
    //app.at("/erprus/ventas/:id/edit").get(venta::ctrl_edit);

    app
}
