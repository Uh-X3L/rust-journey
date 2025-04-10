use rusqlite::{params, Connection, Result};
use crate::utils::hash::hash_owner; 


/// Hashes a string to a SHA-512 hex representation
fn hash_owner(owner: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(owner.as_bytes());
    hex::encode(hasher.finalize())
}

/// Migrates data from old_contract and old_transactions into new schema
pub fn run(conn: &Connection) -> Result<()> {
    println!("🔄 Migrating data from old_contract and old_transactions...");

    // 1. Migrate contracts
    let mut stmt = conn.prepare("SELECT id, owner, balance FROM old_contract")?;
    let contract_rows = stmt.query_map([], |row| {
        let owner: String = row.get(1)?;
        let contract_id = hash_owner(&owner);
        let balance: i64 = row.get(2)?;
        Ok((contract_id, owner, balance))
    })?;

    for result in contract_rows {
        let (contract_id, owner, balance) = result?;
        conn.execute(
            "INSERT OR IGNORE INTO contracts (contract_id, owner, balance) VALUES (?1, ?2, ?3)",
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
        Ok((contract_id, tx_type, amount))
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
