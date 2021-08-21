/// handler comprado
/// autor: fyaniquez
/// fecha: 2021-08-14 23:02:18.883142546 -04:00
///
use crate::models::comprado::Comprado;
use sqlx::{query_as, Postgres, Transaction};
use tide::Error;

/// Crear un registro nuevo
pub async fn hndl_create_tran<'a>(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    estructura: &Comprado,
) -> tide::Result<Comprado> {
    let row: Comprado = query_as!(
        Comprado,
        r#"
INSERT INTO comprados (
    cantidad,costo,subtotal,producto_id,compra_id,created_at,updated_at
)
VALUES ( $1,$2,$3,$4,$5,$6,$7 )
RETURNING id,cantidad,precio,subtotal,producto_id,venta_id,created_at,updated_at
            "#,
        estructura.cantidad,
        estructura.precio,
        estructura.subtotal,
        estructura.producto_id,
        id,
        estructura.created_at,
        estructura.updated_at
    )
    .fetch_one(tx)
    .await
    .map_err(|e| Error::new(409, e))?;
    Ok(row)
}
