pub mod run_migrations;
pub use run_migrations::apply as run_migrations;
pub use run_migrations::reset_specific_migrations;

use rusqlite::{Connection, Result};

/// Establishes a connection to the SQLite database
pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("contract.db")?;
    // Ensure foreign key support is always enabled
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    Ok(conn)
}


