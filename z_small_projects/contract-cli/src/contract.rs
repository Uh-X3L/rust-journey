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
        //add a check to see if the amount is greater than 0
        if amount == 0 {
            println!("⚠️ Deposit failed. Amount must be greater than 0.");
            return Ok(());
        }
        self.balance += amount;
        self.update_balance(conn)?;
        self.log_transaction(conn, amount as i64, "deposit")?;
        println!("✅ Transaction successful!");
        println!("➡️  Action: Deposit");
        println!("💵  Amount: {}", amount);
        println!("📊  New Balance: {}", self.balance);
        Ok(())
    }

    pub fn withdraw(&mut self, conn: &Connection, amount: u64) -> Result<()> {
        //add a check to see if the amount is greater than 0
        if amount == 0 {
            println!("⚠️ Withdrawal failed. Amount must be greater than 0.");
            return Ok(());
        }
        if self.balance >= amount {
            self.balance -= amount;
            self.update_balance(conn)?;
            self.log_transaction(conn, -(amount as i64), "withdraw")?;
            println!("➡️  Action: Withdraw");
            println!("💵  Amount: {}", amount);
            println!("📊  New Balance: {}", self.balance);

        } else {
            println!("❌ Withdrawal failed. Insufficient balance.");
            println!("💰 Current Balance: {}", self.balance);
            println!("🧾 Requested: {}", amount);

        }
        Ok(())
    }

    pub fn show_history(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare(
            "SELECT id, tx_type, amount FROM transactions
             WHERE contract_id = ?1
             ORDER BY id DESC
             LIMIT 5",
        )?;

        let mut rows = stmt.query(params![self.id])?;
        println!("📜 Last 5 transactions for {}:", self.owner);
        while let Some(row) = rows.next()? {
            let tx_id: i32 = row.get(0)?;
            let tx_type: String = row.get(1)?;
            let amount: i64 = row.get(2)?;
            println!("{} | {}: {}", tx_id, tx_type, amount);
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

#[test]
fn test_deposit_increases_balance() {
    let conn = Connection::open_in_memory().unwrap();
    let mut contract = Contract::load_or_create(&conn, 1, "Alice").unwrap();
    contract.deposit(&conn, 100).unwrap();
    assert_eq!(contract.balance, 100);
}

#[test]
fn test_withdraw_decreases_balance() {
    let conn = Connection::open_in_memory().unwrap();
    let mut contract = Contract::load_or_create(&conn, 1, "Alice").unwrap();
    contract.deposit(&conn, 100).unwrap();
    contract.withdraw(&conn, 50).unwrap();
    assert_eq!(contract.balance, 50);
}

#[test]
fn test_withdraw_insufficient_funds() {
    let conn = Connection::open_in_memory().unwrap();
    let mut contract = Contract::load_or_create(&conn, 1, "Alice").unwrap();
    contract.deposit(&conn, 100).unwrap();
    contract.withdraw(&conn, 150).unwrap();
    assert_eq!(contract.balance, 100);
}

#[test]
fn test_show_history() {
    let conn = Connection::open_in_memory().unwrap();
    let mut contract = Contract::load_or_create(&conn, 1, "Alice").unwrap();
    contract.deposit(&conn, 100).unwrap();
    contract.withdraw(&conn, 50).unwrap();
    contract.show_history(&conn).unwrap();
}

#[test]
fn test_status() {
    let conn = Connection::open_in_memory().unwrap();
    let contract = Contract::load_or_create(&conn, 1, "Alice").unwrap();
    contract.status();
}
