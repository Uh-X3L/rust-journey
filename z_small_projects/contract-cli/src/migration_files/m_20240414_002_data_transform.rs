use rusqlite::{params, Connection, Result};
use crate::utils::hash::hash_owner; // Import hash_owner from utils

/// Migrates data from old_contract and old_transactions into new schema
pub fn run(conn: &Connection) -> Result<()> {
    println!("🔄 Migrating data from old_contract and old_transactions...");
    
    // Check if old tables exist
    let old_contract_exists = table_exists(conn, "old_contract")?;
    let old_transactions_exists = table_exists(conn, "old_transactions")?;
    
    if !old_contract_exists {
        println!("⚠️ Table 'old_contract' does not exist. Skipping contract migration.");
        return Ok(());
    }
    
    if !old_transactions_exists {
        println!("⚠️ Table 'old_transactions' does not exist. Skipping transaction migration.");
        return Ok(());
    }

    // 1. Migrate contracts
    let mut stmt = conn.prepare("SELECT contract_id, owner, balance FROM old_contract")?;
    let contract_rows = stmt.query_map([], |row| {
        let owner: String = row.get(1)?;
        let contract_id = hash_owner(&owner);
        let balance: i64 = row.get(2)?;
        Ok((contract_id, owner, balance))
    })?;

    for result in contract_rows {
        let (contract_id, owner, balance) = result?;
        conn.execute(
            "INSERT OR IGNORE INTO contract (contract_id, owner, balance) VALUES (?1, ?2, ?3)",
            params![contract_id, owner, balance],
        )?;
    }

    // 2. Migrate transactions
    let mut tx_stmt = conn.prepare("SELECT contract_id, tx_type, amount FROM old_transactions")?;
    let tx_rows = tx_stmt.query_map([], |row| {
        let old_id: i64 = row.get(0)?;
        let tx_type: String = row.get(1)?;
        let amount: i64 = row.get(2)?;

        // Get owner from old_contract using old ID
        let owner: String = conn.query_row(
            "SELECT owner FROM old_contract WHERE id = ?1",
            [old_id],
            |r| r.get(0),
        )?;

        let contract_id = hash_owner(&owner);
        
        // Handle potentially negative amounts
        let safe_amount = if amount < 0 { 0 } else { amount };
        
        Ok((contract_id, tx_type, safe_amount))
    })?;

    for result in tx_rows {
        let (contract_id, tx_type, amount) = result?;
        conn.execute(
            "INSERT INTO transactions (contract_id, tx_type, amount)
             VALUES (?1, ?2, ?3)",
            params![contract_id, tx_type, amount],
        )?;
    }

    println!("✅ Data migration complete.");
    Ok(())
}

// Helper function to check if a table exists
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
        params![table_name],
        |row| row.get(0)
    )?;
    Ok(count > 0)
}
