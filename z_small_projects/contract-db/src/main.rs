use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Contract {
    id: i32,
    owner: String,
    balance: u64,
}

impl Contract {
    fn new(id: i32, owner: &str, balance: u64) -> Self {
        Self {
            id,
            owner: owner.to_string(),
            balance,
        }
    }

    fn save(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO contract (id, owner, balance) VALUES (?1, ?2, ?3)",
            params![self.id, self.owner, self.balance],
        )?;
        Ok(())
    }

    fn update_balance(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "UPDATE contract SET balance = ?1 WHERE id = ?2",
            params![self.balance, self.id],
        )?;
        Ok(())
    }

    fn log_transaction(&self, conn: &Connection, amount: i64, tx_type: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO transactions (contract_id, tx_type, amount) VALUES (?1, ?2, ?3)",
            params![self.id, tx_type, amount],
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("contract.db")?;

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contract (
            id      INTEGER PRIMARY KEY,
            owner   TEXT NOT NULL,
            balance INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            contract_id  INTEGER NOT NULL,
            tx_type      TEXT NOT NULL,
            amount       INTEGER NOT NULL
        )",
        [],
    )?;

    // Create and store a contract
    let mut contract = Contract::new(1, "alice", 100);
    contract.save(&conn)?;
    contract.log_transaction(&conn, 100, "initial deposit")?;

    // Perform a deposit
    contract.balance += 50;
    contract.update_balance(&conn)?;
    contract.log_transaction(&conn, 50, "deposit")?;

    // Withdraw
    contract.balance -= 30;
    contract.update_balance(&conn)?;
    contract.log_transaction(&conn, -30, "withdrawal")?;

    println!("Final state: {:?}", contract);

    Ok(())
}
