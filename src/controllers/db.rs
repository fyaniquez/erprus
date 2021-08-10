pub struct Connection(Box<postgres::Connection>);
pub struct Transaction<'a>(Box<postgres::transaction::Transaction<'a>>);

pub trait IntoGenericConnection {
    type G: postgres::GenericConnection;
    fn into_generic_connection(&self) -> &Self::G;
}

impl IntoGenericConnection for Connection {
    type G = postgres::Connection;

    fn into_generic_connection(&self) -> &Self::G {
        &self.0
    }
}

impl<'a> IntoGenericConnection for &'a Transaction<'a> {
    type G = postgres::transaction::Transaction<'a>;

    fn into_generic_connection(&self) -> &Self::G {
        &self.0
    }
}

impl Connection {
    pub fn transaction<F, R, E>(self, callback: F) -> Result<R, E>
    where
        F: FnOnce(Transaction) -> Result<R, E>,
    {
        let tx = self.0.transaction().unwrap();
        let res = callback(Transaction(Box::new(tx)))?;
        Ok(res)
    }
}
