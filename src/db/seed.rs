use rusqlite::{Connection, Result};

pub fn seed(conn: &Connection) -> Result<()> {
    let cat_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0))?;
    if cat_count == 0 {
        conn.execute_batch(
            "INSERT INTO categories (name, icon) VALUES
                ('Food', 'utensils'),
                ('Transport', 'bus'),
                ('Shopping', 'shopping-bag'),
                ('Health', 'heart-pulse'),
                ('Entertainment', 'clapperboard'),
                ('Housing', 'house'),
                ('Travel', 'plane'),
                ('Education', 'book-open'),
                ('Gifts', 'gift');",
        )?;
    }

    let acct_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM accounts", [], |r| r.get(0))?;
    if acct_count == 0 {
        conn.execute_batch(
            "INSERT INTO accounts (name, starting_balance, icon, currency) VALUES
                ('Cash', 0.0, 'banknote', 'USD'),
                ('Bank', 0.0, 'landmark', 'USD'),
                ('Credit Card', 0.0, 'credit-card', 'USD');",
        )?;
    }

    Ok(())
}
