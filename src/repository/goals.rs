use crate::db::DbConnection;
use crate::models::model::Goals;
use rusqlite::{params, Result};

pub fn get_all(db: &DbConnection) -> Result<Vec<Goals>> {
    let conn = db.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT id, name, target, current, deadline FROM goals")?;
    let rows = stmt.query_map([], |row| {
        Ok(Goals {
            id: row.get(0)?,
            name: row.get(1)?,
            target: row.get::<_, f64>(2)? as f32,
            current: row.get::<_, f64>(3)? as f32,
            deadline: row.get::<_, Option<f64>>(4)?.map(|v| v as f32),
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Goals> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, name, target, current, deadline FROM goals WHERE id = ?1",
        [id],
        |row| {
            Ok(Goals {
                id: row.get(0)?,
                name: row.get(1)?,
                target: row.get::<_, f64>(2)? as f32,
                current: row.get::<_, f64>(3)? as f32,
                deadline: row.get::<_, Option<f64>>(4)?.map(|v| v as f32),
            })
        },
    )
}

pub fn insert(db: &DbConnection, goal: &Goals) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO goals (name, target, current, deadline) VALUES (?1, ?2, ?3, ?4)",
        params![
            goal.name,
            goal.target as f64,
            goal.current as f64,
            goal.deadline.map(|v| v as f64)
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, goal: &Goals) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE goals SET name = ?1, target = ?2, current = ?3, deadline = ?4 WHERE id = ?5",
        params![
            goal.name,
            goal.target as f64,
            goal.current as f64,
            goal.deadline.map(|v| v as f64),
            goal.id
        ],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM goals WHERE id = ?1", [id])?;
    Ok(())
}
