use crate::db::DbConnection;
use crate::models::model::{TransactionType, Transactions};
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Result};

fn tx_type_to_str(tx: &TransactionType) -> &'static str {
    match tx {
        TransactionType::Income => "income",
        TransactionType::Expense => "expense",
        TransactionType::Transfer => "transfer",
    }
}

fn tx_type_from_str(s: &str) -> TransactionType {
    match s {
        "income" => TransactionType::Income,
        "transfer" => TransactionType::Transfer,
        _ => TransactionType::Expense,
    }
}

pub fn get_all(db: &DbConnection) -> Result<Vec<Transactions>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till FROM transactions",
    )?;
    let rows = stmt.query_map([], |row| {
        let date_str: String = row.get(3)?;
        let time_str: String = row.get(4)?;
        let type_str: String = row.get(5)?;
        Ok(Transactions {
            id: row.get(0)?,
            title: row.get(1)?,
            amount: row.get::<_, f64>(2)? as f32,
            tx_date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default(),
            tx_time: NaiveTime::parse_from_str(&time_str, "%H:%M:%S").unwrap_or_default(),
            tx_type: tx_type_from_str(&type_str),
            category: row.get(6)?,
            account: row.get(7)?,
            description: row.get(8)?,
            goal_id:        row.get(9)?,
            loan_id:        row.get(10)?,
            frequency:      row.get(11)?,
            recurring_till: row.get(12)?,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Transactions> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till FROM transactions WHERE id = ?1",
        [id],
        |row| {
            let date_str: String = row.get(3)?;
            let time_str: String = row.get(4)?;
            let type_str: String = row.get(5)?;
            Ok(Transactions {
                id: row.get(0)?,
                title: row.get(1)?,
                amount: row.get::<_, f64>(2)? as f32,
                tx_date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default(),
                tx_time: NaiveTime::parse_from_str(&time_str, "%H:%M:%S").unwrap_or_default(),
                tx_type: tx_type_from_str(&type_str),
                category: row.get(6)?,
                account: row.get(7)?,
                description: row.get(8)?,
                goal_id:        row.get(9)?,
                loan_id:        row.get(10)?,
                frequency:      row.get(11)?,
                recurring_till: row.get(12)?,
            })
        },
    )
}

pub fn insert(db: &DbConnection, tx: &Transactions) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO transactions (title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            tx.title,
            tx.amount as f64,
            tx.tx_date.format("%Y-%m-%d").to_string(),
            tx.tx_time.format("%H:%M:%S").to_string(),
            tx_type_to_str(&tx.tx_type),
            tx.category,
            tx.account,
            tx.description,
            tx.goal_id,
            tx.loan_id,
            tx.frequency,
            tx.recurring_till,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, tx: &Transactions) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE transactions SET title = ?1, amount = ?2, tx_date = ?3, tx_time = ?4,
         tx_type = ?5, category = ?6, account = ?7, description = ?8,
         goal_id = ?9, loan_id = ?10, frequency = ?11, recurring_till = ?12 WHERE id = ?13",
        params![
            tx.title,
            tx.amount as f64,
            tx.tx_date.format("%Y-%m-%d").to_string(),
            tx.tx_time.format("%H:%M:%S").to_string(),
            tx_type_to_str(&tx.tx_type),
            tx.category,
            tx.account,
            tx.description,
            tx.goal_id,
            tx.loan_id,
            tx.frequency,
            tx.recurring_till,
            tx.id,
        ],
    )?;
    Ok(())
}

pub fn get_by_month(db: &DbConnection, month: u32, year: i32) -> Result<Vec<Transactions>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till
         FROM transactions
         WHERE frequency IS NULL
           AND strftime('%Y', tx_date) = ?1
           AND strftime('%m', tx_date) = ?2
         ORDER BY tx_date DESC, tx_time DESC",
    )?;
    let rows = stmt.query_map(
        rusqlite::params![format!("{year:04}"), format!("{month:02}")],
        |row| {
            let date_str: String = row.get(3)?;
            let time_str: String = row.get(4)?;
            let type_str: String = row.get(5)?;
            Ok(Transactions {
                id: row.get(0)?,
                title: row.get(1)?,
                amount: row.get::<_, f64>(2)? as f32,
                tx_date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default(),
                tx_time: NaiveTime::parse_from_str(&time_str, "%H:%M:%S").unwrap_or_default(),
                tx_type: tx_type_from_str(&type_str),
                category: row.get(6)?,
                account: row.get(7)?,
                description: row.get(8)?,
                goal_id:        row.get(9)?,
                loan_id:        row.get(10)?,
                frequency:      row.get(11)?,
                recurring_till: row.get(12)?,
            })
        },
    )?;
    rows.collect()
}

pub fn get_all_recurring(db: &DbConnection) -> Result<Vec<Transactions>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till
         FROM transactions
         WHERE frequency IS NOT NULL",
    )?;
    let rows = stmt.query_map([], |row| {
        let date_str: String = row.get(3)?;
        let time_str: String = row.get(4)?;
        let type_str: String = row.get(5)?;
        Ok(Transactions {
            id: row.get(0)?,
            title: row.get(1)?,
            amount: row.get::<_, f64>(2)? as f32,
            tx_date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap_or_default(),
            tx_time: NaiveTime::parse_from_str(&time_str, "%H:%M:%S").unwrap_or_default(),
            tx_type: tx_type_from_str(&type_str),
            category: row.get(6)?,
            account: row.get(7)?,
            description: row.get(8)?,
            goal_id:        row.get(9)?,
            loan_id:        row.get(10)?,
            frequency:      row.get(11)?,
            recurring_till: row.get(12)?,
        })
    })?;
    rows.collect()
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM transactions WHERE id = ?1", [id])?;
    Ok(())
}
