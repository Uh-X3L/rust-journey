use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Contract {
    pub id: i32,
    pub owner: String,
    pub balance: u64,
}

impl Contract {
    pub fn load_or_create(conn: &Connection, id: i32, owner: &str) -> Result<Self> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS contract (
                id      INTEGER PRIMARY KEY,
                owner   TEXT NOT NULL,
                balance INTEGER NOT NULL
            )", [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                contract_id  INTEGER NOT NULL,
                tx_type      TEXT NOT NULL,
                amount       INTEGER NOT NULL
            )", [],
        )?;

        let mut stmt = conn.prepare("SELECT owner, balance FROM contract WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Self {
                id,
                owner: row.get(0)?,
                balance: row.get(1)?,
            })
        } else {
            conn.execute(
                "INSERT INTO contract (id, owner, balance) VALUES (?1, ?2, ?3)",
                params![id, owner, 0],
            )?;
            Ok(Self {
                id,
                owner: owner.to_string(),
                balance: 0,
            })
        }
    }

    pub fn deposit(&mut self, conn: &Connection, amount: u64) -> Result<()> {
        self.balance += amount;
        self.update_balance(conn)?;
        self.log_transaction(conn, amount as i64, "deposit")?;
        println!("✅ Deposited {}. New balance: {}", amount, self.balance);
        Ok(())
    }

    pub fn withdraw(&mut self, conn: &Connection, amount: u64) -> Result<()> {
        if self.balance >= amount {
            self.balance -= amount;
            self.update_balance(conn)?;
            self.log_transaction(conn, -(amount as i64), "withdraw")?;
            println!("✅ Withdrew {}. New balance: {}", amount, self.balance);
        } else {
            println!("❌ Insufficient funds. Balance: {}", self.balance);
        }
        Ok(())
    }

    pub fn show_history(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare(
            "SELECT tx_type, amount FROM transactions
             WHERE contract_id = ?1
             ORDER BY id DESC
             LIMIT 5",
        )?;

        let mut rows = stmt.query(params![self.id])?;
        println!("📜 Last 5 transactions:");
        while let Some(row) = rows.next()? {
            let tx_type: String = row.get(0)?;
            let amount: i64 = row.get(1)?;
            println!("- {}: {}", tx_type, amount);
        }
        Ok(())
    }

    pub fn status(&self) {
        println!("👤 Owner: {}", self.owner);
        println!("💰 Balance: {}", self.balance);
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
            "INSERT INTO transactions (contract_id, tx_type, amount)
             VALUES (?1, ?2, ?3)",
            params![self.id, tx_type, amount],
        )?;
        Ok(())
    }
}
