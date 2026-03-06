use crate::db::DbConnection;
use crate::models::model::Account;
use rusqlite::{params, Result};

pub fn get_all(db: &DbConnection) -> Result<Vec<Account>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, starting_balance, icon, currency FROM accounts",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            starting_balance: row.get::<_, f64>(2)? as f32,
            icon: row.get(3)?,
            currency: row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Account> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, name, starting_balance, icon, currency FROM accounts WHERE id = ?1",
        [id],
        |row| {
            Ok(Account {
                id: row.get(0)?,
                name: row.get(1)?,
                starting_balance: row.get::<_, f64>(2)? as f32,
                icon: row.get(3)?,
                currency: row.get(4)?,
            })
        },
    )
}

pub fn insert(db: &DbConnection, account: &Account) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO accounts (name, starting_balance, icon, currency) VALUES (?1, ?2, ?3, ?4)",
        params![
            account.name,
            account.starting_balance as f64,
            account.icon,
            account.currency
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, account: &Account) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE accounts SET name = ?1, starting_balance = ?2, icon = ?3, currency = ?4 WHERE id = ?5",
        params![
            account.name,
            account.starting_balance as f64,
            account.icon,
            account.currency,
            account.id
        ],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM accounts WHERE id = ?1", [id])?;
    Ok(())
}
