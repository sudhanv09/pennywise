use rusqlite::{Connection, Result};

pub fn seed(conn: &Connection) -> Result<()> {
    let cat_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0))?;
    if cat_count == 0 {
        conn.execute_batch(
            "INSERT INTO categories (name, icon) VALUES
                ('Food', '🍔'),
                ('Transport', '🚌'),
                ('Shopping', '🛍'),
                ('Health', '💊'),
                ('Entertainment', '🎬'),
                ('Housing', '🏠'),
                ('Travel', '✈️'),
                ('Education', '📚'),
                ('Gifts', '🎁');",
        )?;
    }

    let acct_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM accounts", [], |r| r.get(0))?;
    if acct_count == 0 {
        conn.execute_batch(
            "INSERT INTO accounts (name, starting_balance, icon, currency) VALUES
                ('Cash', 0.0, '💵', 'USD'),
                ('Bank', 0.0, '🏦', 'USD'),
                ('Credit Card', 0.0, '💳', 'USD');",
        )?;
    }

    Ok(())
}
