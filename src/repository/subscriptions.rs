use crate::db::DbConnection;
use crate::models::model::{BillingCycle, Subscription, TransactionType, Transactions};
use crate::repository::transactions as tx_repo;
use chrono::{Datelike, Months, NaiveDate, NaiveTime};
use rusqlite::{params, Result};

fn billing_cycle_to_str(cycle: &BillingCycle) -> &'static str {
    match cycle {
        BillingCycle::Daily   => "daily",
        BillingCycle::Weekly  => "weekly",
        BillingCycle::Monthly => "monthly",
        BillingCycle::Yearly  => "yearly",
    }
}

fn billing_cycle_from_str(s: &str) -> BillingCycle {
    match s {
        "daily"   => BillingCycle::Daily,
        "weekly"  => BillingCycle::Weekly,
        "yearly"  => BillingCycle::Yearly,
        _         => BillingCycle::Monthly,
    }
}

pub fn get_all(db: &DbConnection) -> Result<Vec<Subscription>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, amount, billing_cycle, start_date, end_date, category_id, account_id FROM subscriptions"
    )?;
    let rows = stmt.query_map([], |row| {
        let cycle_str: String = row.get(3)?;
        Ok(Subscription {
            id: row.get(0)?,
            name: row.get(1)?,
            amount: row.get::<_, f64>(2)? as f32,
            billing_cycle: billing_cycle_from_str(&cycle_str),
            start_date: row.get(4)?,
            end_date: row.get(5)?,
            category_id: row.get(6)?,
            account_id: row.get(7)?,
        })
    })?;
    rows.collect()
}

pub fn insert(db: &DbConnection, sub: &Subscription) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO subscriptions (name, amount, billing_cycle, start_date, end_date, category_id, account_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            sub.name,
            sub.amount as f64,
            billing_cycle_to_str(&sub.billing_cycle),
            sub.start_date,
            sub.end_date,
            sub.category_id,
            sub.account_id,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, sub: &Subscription) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE subscriptions SET name = ?1, amount = ?2, billing_cycle = ?3,
         start_date = ?4, end_date = ?5, category_id = ?6, account_id = ?7
         WHERE id = ?8",
        params![
            sub.name,
            sub.amount as f64,
            billing_cycle_to_str(&sub.billing_cycle),
            sub.start_date,
            sub.end_date,
            sub.category_id,
            sub.account_id,
            sub.id,
        ],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM subscriptions WHERE id = ?1", [id])?;
    Ok(())
}

fn next_date(current: NaiveDate, cycle: &BillingCycle) -> Option<NaiveDate> {
    match cycle {
        BillingCycle::Daily   => current.succ_opt(),
        BillingCycle::Weekly  => current.checked_add_days(chrono::Days::new(7)),
        BillingCycle::Monthly => current.checked_add_months(Months::new(1)),
        BillingCycle::Yearly  => current.checked_add_months(Months::new(12)),
    }
}

pub fn generate_transactions(db: &DbConnection, sub_id: i32, sub: &Subscription) {
    let start = match NaiveDate::parse_from_str(&sub.start_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return,
    };
    let end = match NaiveDate::parse_from_str(&sub.end_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return,
    };

    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let mut date = start;

    while date <= end {
        let tx = Transactions {
            id: -1,
            title: sub.name.clone(),
            amount: sub.amount,
            tx_date: date,
            tx_time: midnight,
            tx_type: TransactionType::Expense,
            category: sub.category_id as i16,
            account: sub.account_id as i16,
            description: format!("Subscription: {}", sub.name),
            goal_id: None,
            loan_id: None,
            frequency: None,
            recurring_till: None,
            to_account: None,
            subscription_id: Some(sub_id),
        };
        let _ = tx_repo::insert(db, &tx);

        match next_date(date, &sub.billing_cycle) {
            Some(d) => date = d,
            None => break,
        }
    }
}
