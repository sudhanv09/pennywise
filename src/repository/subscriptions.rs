use crate::db::DbConnection;
use crate::models::model::{BillingCycle, Subscription};
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
        "weekly" => BillingCycle::Weekly,
        "yearly" => BillingCycle::Yearly,
        _ => BillingCycle::Monthly,
    }
}

pub fn get_all(db: &DbConnection) -> Result<Vec<Subscription>> {
    let conn = db.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT id, name, billing_cycle, next_billing FROM subscriptions")?;
    let rows = stmt.query_map([], |row| {
        let cycle_str: String = row.get(2)?;
        Ok(Subscription {
            id: row.get(0)?,
            name: row.get(1)?,
            billing_cycle: billing_cycle_from_str(&cycle_str),
            next_billing: row.get::<_, f64>(3)? as f32,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Subscription> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, name, billing_cycle, next_billing FROM subscriptions WHERE id = ?1",
        [id],
        |row| {
            let cycle_str: String = row.get(2)?;
            Ok(Subscription {
                id: row.get(0)?,
                name: row.get(1)?,
                billing_cycle: billing_cycle_from_str(&cycle_str),
                next_billing: row.get::<_, f64>(3)? as f32,
            })
        },
    )
}

pub fn insert(db: &DbConnection, sub: &Subscription) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO subscriptions (name, billing_cycle, next_billing) VALUES (?1, ?2, ?3)",
        params![
            sub.name,
            billing_cycle_to_str(&sub.billing_cycle),
            sub.next_billing as f64
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, sub: &Subscription) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE subscriptions SET name = ?1, billing_cycle = ?2, next_billing = ?3 WHERE id = ?4",
        params![
            sub.name,
            billing_cycle_to_str(&sub.billing_cycle),
            sub.next_billing as f64,
            sub.id
        ],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM subscriptions WHERE id = ?1", [id])?;
    Ok(())
}
