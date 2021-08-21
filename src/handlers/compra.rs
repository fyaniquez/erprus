use crate::handlers::comprado;
/// handler compra
/// autor: fyaniquez
/// fecha: 2021-08-02 23:02:06.536948792 -04:00
///
use crate::models::compra::Compra;
use crate::models::comprado::Comprado;
use sqlx::{query_as, PgPool, Postgres, Transaction};
use tide::Error;

const SELECT: &str = r#"
SELECT id, fecha, total, pago, factura, numero, observaciones, 
    distribuidora_id, empleado_id, created_at, updated_at
FROM compras"#;

const INSERT: &str = r#"
INSERT INTO compras (
    fecha, total, pago, factura, numero, observaciones, 
    distribuidora_id, empleado_id, created_at, updated_at
)
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )
RETURNING id, fecha, total, pago, factura, numero, observaciones, 
distribuidora_id, empleado_id, created_at, updated_at"#;
/// Crea un registro de acuerdo a los parametros
pub async fn hndl_create<'a>(
    db_pool: &PgPool,
    compra: Compra<'a>,
    comprados: Vec<Comprado>,
) -> tide::Result<Compra<'a>> {
    // inicia la transaccion
    let mut tx = db_pool.begin().await?;
    let compra: Compra = hndl_create_tran(&mut tx, compra).await?;
    for i in 0..comprados.len() {
        let resv = match comprado::hndl_create_tran(&mut tx, compra.id, &comprados[i]).await {
            Ok(_row) => (),
            Err(err) => return Err(err),
        };
    }
    tx.commit().await?;
    Ok(compra)
}

/// Crear un registro nuevo
async fn hndl_create_tran<'a>(
    db: &mut Transaction<'a, Postgres>,
    compra: Compra<'a>,
) -> tide::Result<Compra<'a>> {
    let row: Compra = query_as(INSERT)
        .bind(compra.fecha)
        .bind(compra.total)
        .bind(compra.pago)
        .bind(compra.factura)
        .bind(compra.numero)
        .bind(compra.observaciones)
        .bind(compra.distribuidora_id)
        .bind(compra.empleado_id)
        .bind(compra.created_at)
        .bind(compra.updated_at)
        .fetch_one(db)
        .await
        .map_err(|e| Error::new(409, e))?;

    Ok(row)
}
