use crate::db::DbConnection;
use crate::models::model::Loans;
use rusqlite::{params, Result};

pub fn get_all(db: &DbConnection) -> Result<Vec<Loans>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, total_amount, paid_amount, due, is_lender FROM loans",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Loans {
            id: row.get(0)?,
            name: row.get(1)?,
            total_amount: row.get::<_, f64>(2)? as f32,
            paid_amount: row.get::<_, f64>(3)? as f32,
            due: row.get::<_, f64>(4)? as f32,
            is_lender: row.get::<_, i32>(5)? != 0,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Loans> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, name, total_amount, paid_amount, due, is_lender FROM loans WHERE id = ?1",
        [id],
        |row| {
            Ok(Loans {
                id: row.get(0)?,
                name: row.get(1)?,
                total_amount: row.get::<_, f64>(2)? as f32,
                paid_amount: row.get::<_, f64>(3)? as f32,
                due: row.get::<_, f64>(4)? as f32,
                is_lender: row.get::<_, i32>(5)? != 0,
            })
        },
    )
}

pub fn insert(db: &DbConnection, loan: &Loans) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO loans (name, total_amount, paid_amount, due, is_lender) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            loan.name,
            loan.total_amount as f64,
            loan.paid_amount as f64,
            loan.due as f64,
            loan.is_lender as i32
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, loan: &Loans) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE loans SET name = ?1, total_amount = ?2, paid_amount = ?3, due = ?4, is_lender = ?5 WHERE id = ?6",
        params![
            loan.name,
            loan.total_amount as f64,
            loan.paid_amount as f64,
            loan.due as f64,
            loan.is_lender as i32,
            loan.id
        ],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM loans WHERE id = ?1", [id])?;
    Ok(())
}
