use crate::models::vendido::Vendido;
/// handler venta
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:06.536948792 -04:00
///
use crate::models::venta::Venta;
use sqlx::{query_as, PgPool, Postgres, Transaction};
use tide::Error;

/// Obtener un registros de acuerdo a su id
pub async fn hndl_get(db_pool: &PgPool, id: i64) -> tide::Result<Option<Venta>> {
    let row = sqlx::query_as!(
        Venta,
        r#"
SELECT id,fecha,total,pago,cliente_id,cajero_id,created_at,updated_at
FROM ventas
WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}
pub async fn hndl_create(
    db_pool: &PgPool,
    venta: Venta,
    vendidos: Vec<Vendido>,
) -> tide::Result<Venta> {
    let mut errores = String::new();
    // inicia la transaccion
    let mut tx = db_pool.begin().await?;
    let venta: Venta = hndl_create_tran(&mut tx, venta).await.unwrap();
    for i in 0..vendidos.len() {
        let resv = match crate::handlers::vendido::hndl_create_tran(&mut tx, venta.id, &vendidos[i])
            .await
        {
            Ok(_row) => (),
            Err(err) => (errores.push_str(&err.to_string())),
        };
    }
    tx.commit().await?;
    Ok(venta)
}
/// Crear un registro nuevo
pub async fn hndl_create_tran<'a>(
    db: &mut Transaction<'_, Postgres>,
    venta: Venta,
) -> tide::Result<Venta> {
    let row: Venta = query_as!(
        Venta,
        r#"
INSERT INTO ventas (
    fecha,total,pago,cliente_id,cajero_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4,$5,$6,$7 )
RETURNING id,fecha,total,pago,cliente_id,cajero_id,created_at,updated_at
            "#,
        venta.fecha,
        venta.total,
        venta.pago,
        venta.cliente_id,
        venta.cajero_id,
        venta.created_at,
        venta.updated_at
    )
    .fetch_one(db)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(row)
}
