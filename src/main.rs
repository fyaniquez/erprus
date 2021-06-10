mod controllers;
mod handlers;
mod models;

use controllers::caja::controller_list;
use sqlx::PgPool;
use sqlx::Pool;
use tide::http::cookies::SameSite;
use tide::prelude::*;
use tide::Server;

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

    app.with(
        tide::sessions::SessionMiddleware::new(
            tide::sessions::MemoryStore::new(),
            std::env::var("TIDE_SECRET")
                .expect("Please provide a TIDE_SECRET value of at least 32 bytes")
                .as_bytes(),
        )
        .with_same_site_policy(SameSite::Lax),
    );

    // api
    app.at("/cajas").get(controller_list);

    app
}
