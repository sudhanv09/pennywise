pub mod seed;
pub mod tables;

use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

pub fn db_path() -> PathBuf {
    let mut dir = cache_dir::get_data_dir().expect("no writable app data dir");
    dir.push("pennywise.db");
    dir
}

pub fn init() -> Result<DbConnection> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| rusqlite::Error::InvalidPath(path.clone()))?;
    }

    let conn = Connection::open(&path)?;
    conn.execute_batch(
        r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            "#,
    )?;
    tables::create_all(&conn)?;
    seed::seed(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}
