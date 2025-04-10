
use rusqlite::{Connection, Result};

/// Establishes a connection to the SQLite database
pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("contract.db")?;
    // Ensure foreign key support is always enabled
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    Ok(conn)
}
