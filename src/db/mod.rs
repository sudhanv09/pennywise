pub mod tables;

use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

pub fn init(path: &str) -> Result<DbConnection> {
    let conn = Connection::open(path)?;
    tables::create_all(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}
