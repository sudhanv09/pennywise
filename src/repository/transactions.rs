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
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till, to_account, subscription_id FROM transactions",
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
            to_account:     row.get(13)?,
            subscription_id: row.get(14)?,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Transactions> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till, to_account, subscription_id FROM transactions WHERE id = ?1",
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
                to_account:     row.get(13)?,
                subscription_id: row.get(14)?,
            })
        },
    )
}

pub fn insert(db: &DbConnection, tx: &Transactions) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO transactions (title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till, to_account, subscription_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
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
            tx.to_account,
            tx.subscription_id,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, tx: &Transactions) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE transactions SET title = ?1, amount = ?2, tx_date = ?3, tx_time = ?4,
         tx_type = ?5, category = ?6, account = ?7, description = ?8,
         goal_id = ?9, loan_id = ?10, frequency = ?11, recurring_till = ?12,
         to_account = ?13, subscription_id = ?14 WHERE id = ?15",
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
            tx.to_account,
            tx.subscription_id,
            tx.id,
        ],
    )?;
    Ok(())
}

pub fn get_by_month(db: &DbConnection, month: u32, year: i32) -> Result<Vec<Transactions>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till, to_account, subscription_id
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
                to_account:     row.get(13)?,
                subscription_id: row.get(14)?,
            })
        },
    )?;
    rows.collect()
}

pub fn get_all_recurring(db: &DbConnection) -> Result<Vec<Transactions>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, amount, tx_date, tx_time, tx_type, category, account, description, goal_id, loan_id, frequency, recurring_till, to_account, subscription_id
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
            to_account:     row.get(13)?,
            subscription_id: row.get(14)?,
        })
    })?;
    rows.collect()
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM transactions WHERE id = ?1", [id])?;
    Ok(())
}

pub fn delete_by_subscription(db: &DbConnection, subscription_id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM transactions WHERE subscription_id = ?1", [subscription_id])?;
    Ok(())
}

pub fn sum_for_goal(db: &DbConnection, goal_id: i32) -> Result<f32> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE goal_id = ?1",
        [goal_id],
        |row| Ok(row.get::<_, f64>(0)? as f32),
    )
}

pub fn sum_for_loan(db: &DbConnection, loan_id: i32) -> Result<f32> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE loan_id = ?1",
        [loan_id],
        |row| Ok(row.get::<_, f64>(0)? as f32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbConnection;
    use crate::db::tables;
    use crate::models::model::{Goals, Loans, TransactionType, Transactions};
    use crate::repository::{goals as goal_repo, loans as loan_repo};
    use chrono::{NaiveDate, NaiveTime};
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    fn test_db() -> DbConnection {
        let conn = Connection::open_in_memory().unwrap();
        tables::create_all(&conn).unwrap();
        tables::migrate(&conn).unwrap();
        Arc::new(Mutex::new(conn))
    }

    fn make_tx(goal_id: Option<i32>, loan_id: Option<i32>, amount: f32) -> Transactions {
        Transactions {
            id: 0,
            title: "test".into(),
            amount,
            tx_date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            tx_time: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            tx_type: TransactionType::Expense,
            category: 1,
            account: 1,
            description: String::new(),
            goal_id,
            loan_id,
            frequency: None,
            recurring_till: None,
            to_account: None,
            subscription_id: None,
        }
    }

    #[test]
    fn sum_for_goal_no_transactions() {
        let db = test_db();
        assert_eq!(sum_for_goal(&db, 999).unwrap(), 0.0);
    }

    #[test]
    fn sum_for_goal_with_transactions() {
        let db = test_db();
        let goal = Goals { id: 0, name: "Save".into(), target: 1000.0, current: 0.0, deadline: None };
        let gid = goal_repo::insert(&db, &goal).unwrap() as i32;

        insert(&db, &make_tx(Some(gid), None, 200.0)).unwrap();
        insert(&db, &make_tx(Some(gid), None, 350.0)).unwrap();

        let total = sum_for_goal(&db, gid).unwrap();
        assert!((total - 550.0).abs() < 0.01, "expected 550, got {}", total);
    }

    #[test]
    fn sum_for_goal_ignores_other_goals() {
        let db = test_db();
        let g1 = Goals { id: 0, name: "A".into(), target: 500.0, current: 0.0, deadline: None };
        let g2 = Goals { id: 0, name: "B".into(), target: 500.0, current: 0.0, deadline: None };
        let gid1 = goal_repo::insert(&db, &g1).unwrap() as i32;
        let gid2 = goal_repo::insert(&db, &g2).unwrap() as i32;

        insert(&db, &make_tx(Some(gid1), None, 100.0)).unwrap();
        insert(&db, &make_tx(Some(gid2), None, 200.0)).unwrap();
        insert(&db, &make_tx(None, None, 999.0)).unwrap(); // untagged

        assert!((sum_for_goal(&db, gid1).unwrap() - 100.0).abs() < 0.01);
        assert!((sum_for_goal(&db, gid2).unwrap() - 200.0).abs() < 0.01);
    }

    #[test]
    fn sum_for_loan_no_transactions() {
        let db = test_db();
        assert_eq!(sum_for_loan(&db, 999).unwrap(), 0.0);
    }

    #[test]
    fn sum_for_loan_with_transactions() {
        let db = test_db();
        let loan = Loans { id: 0, name: "Car".into(), total_amount: 5000.0, paid_amount: 0.0, due: 740000.0, is_lender: false };
        let lid = loan_repo::insert(&db, &loan).unwrap() as i32;

        insert(&db, &make_tx(None, Some(lid), 500.0)).unwrap();
        insert(&db, &make_tx(None, Some(lid), 750.0)).unwrap();

        let total = sum_for_loan(&db, lid).unwrap();
        assert!((total - 1250.0).abs() < 0.01, "expected 1250, got {}", total);
    }

    #[test]
    fn sum_for_loan_ignores_other_loans() {
        let db = test_db();
        let l1 = Loans { id: 0, name: "A".into(), total_amount: 1000.0, paid_amount: 0.0, due: 740000.0, is_lender: false };
        let l2 = Loans { id: 0, name: "B".into(), total_amount: 2000.0, paid_amount: 0.0, due: 740000.0, is_lender: false };
        let lid1 = loan_repo::insert(&db, &l1).unwrap() as i32;
        let lid2 = loan_repo::insert(&db, &l2).unwrap() as i32;

        insert(&db, &make_tx(None, Some(lid1), 100.0)).unwrap();
        insert(&db, &make_tx(None, Some(lid2), 200.0)).unwrap();

        assert!((sum_for_loan(&db, lid1).unwrap() - 100.0).abs() < 0.01);
        assert!((sum_for_loan(&db, lid2).unwrap() - 200.0).abs() < 0.01);
    }
}
