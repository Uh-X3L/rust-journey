use crate::migration_files::m_20240414_002_data_transform;
use anyhow::Result;
use rusqlite::{Connection, params};
use std::fs;
use std::path::Path;
use std::time::Instant; // Import the migration module

// New function to reset specific migrations
pub fn reset_specific_migrations(conn: &Connection, migration_names: &[&str]) -> Result<()> {
    // Delete the specified migrations from the migrations table
    for name in migration_names {
        println!("🔄 Resetting migration: {}", name);
        conn.execute("DELETE FROM migrations WHERE filename = ?1", params![name])
            .map_err(anyhow::Error::from)?;
    }

    Ok(())
}

pub fn apply(conn: &Connection, filename: &str) -> Result<()> {
    // ✅ Create tracking table for applied migrations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            filename TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            status TEXT NOT NULL DEFAULT 'pending',
            error TEXT,
            duration INTEGER
        )",
        [],
    )
    .map_err(anyhow::Error::from)?;

    // 🛑 Skip if already applied
    let already_run: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM migrations WHERE filename = ?1",
            params![&filename],
            |row| row.get(0),
        )
        .map_err(anyhow::Error::from)?;

    if already_run > 0 {
        return Ok(());
    }

    let start_time = Instant::now();
    let mut status = "success";

    // Create a Path from the filename
    let path = Path::new(filename);

    // 🧠 Dispatch based on file extension
    let result: Result<()> = match path.extension().and_then(|ext| ext.to_str()) {
        Some("sql") => {
            println!("🧱 Running SQL migration: {}", filename);
            let sql = fs::read_to_string(filename).map_err(anyhow::Error::from)?;
            conn.execute_batch(&sql).map_err(anyhow::Error::from)
        }
        Some("rs") => {
            println!("🧪 Running Rust migration: {}", filename);
            // Dynamically match migration module by filename
            match filename.strip_suffix(".rs") {
                Some("m_20240414_002_data_transform") => {
                    m_20240414_002_data_transform::run(conn).map_err(anyhow::Error::from)
                }
                _ => Err(anyhow::anyhow!("Unknown Rust migration")),
            }
        }
        _ => {
            status = "skipped";
            println!("⚠️ Skipping unsupported file: {}", filename);
            Ok(())
        }
    };

    // ⏱ Track execution time & errors
    let duration = start_time.elapsed().as_millis() as i64;

    let error_message: Option<String> = if let Err(ref e) = result {
        status = "failed";
        Some(format!("{}", e))
    } else {
        None
    };

    let error_msg = error_message.as_deref();

    // 📝 Log result into migrations table
    conn.execute(
        "INSERT INTO migrations (filename, status, error, duration) VALUES (?1, ?2, ?3, ?4)",
        params![&filename, status, error_msg, duration],
    )
    .map_err(anyhow::Error::from)?;

    // 🟩🟥 Print migration status
    match status {
        "success" => println!("✅ Completed in {} ms", duration),
        "failed" => println!("❌ Failed: {} — {}", filename, error_message.unwrap()),
        _ => {}
    }

    result
}
