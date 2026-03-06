use crate::db::DbConnection;
use crate::models::model::Category;
use rusqlite::{params, Result};

pub fn get_all(db: &DbConnection) -> Result<Vec<Category>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, icon FROM categories")?;
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        })
    })?;
    rows.collect()
}

pub fn get_by_id(db: &DbConnection, id: i32) -> Result<Category> {
    let conn = db.lock().unwrap();
    conn.query_row(
        "SELECT id, name, icon FROM categories WHERE id = ?1",
        [id],
        |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
            })
        },
    )
}

pub fn insert(db: &DbConnection, category: &Category) -> Result<i64> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO categories (name, icon) VALUES (?1, ?2)",
        params![category.name, category.icon],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update(db: &DbConnection, category: &Category) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE categories SET name = ?1, icon = ?2 WHERE id = ?3",
        params![category.name, category.icon, category.id],
    )?;
    Ok(())
}

pub fn delete(db: &DbConnection, id: i32) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM categories WHERE id = ?1", [id])?;
    Ok(())
}
