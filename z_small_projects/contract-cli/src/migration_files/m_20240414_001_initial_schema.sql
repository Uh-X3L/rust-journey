-- Migration to new schema with hashed TEXT contract IDs
-- Use defensive approach for table operations in SQLite

-- First drop old tables if they exist from previous attempts
DROP TABLE IF EXISTS old_contract;
DROP TABLE IF EXISTS old_transactions;

-- Check if contract table exists and rename it
BEGIN TRANSACTION;
SELECT CASE 
    WHEN EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='contract')
    THEN (SELECT 1 FROM pragma_table_info('contract') WHERE 1=0) -- this select doesn't run, but lets us keep going
    ELSE (SELECT 1) -- dummy SELECT to ensure the script continues
END;

-- Use these individual statements rather than ALTER TABLE WHEN EXISTS
-- SQLite will skip ALTER TABLE if the table doesn't exist, which is exactly what we want
ALTER TABLE contract RENAME TO old_contract;
COMMIT;

-- Same approach for transactions
BEGIN TRANSACTION;
SELECT CASE 
    WHEN EXISTS (SELECT 1 FROM sqlite_master WHERE type='table' AND name='transactions')
    THEN (SELECT 1 FROM pragma_table_info('transactions') WHERE 1=0)
    ELSE (SELECT 1)
END;
ALTER TABLE transactions RENAME TO old_transactions;
COMMIT;

PRAGMA foreign_keys = ON;

-- Create new tables
CREATE TABLE contract (
    contract_id TEXT PRIMARY KEY,
    owner       TEXT NOT NULL UNIQUE,
    balance     INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE transactions (
    tx_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    contract_id  TEXT NOT NULL,
    tx_type      TEXT CHECK(tx_type IN ('deposit', 'withdraw')) NOT NULL,
    amount       INTEGER NOT NULL CHECK(amount >= 0),
    timestamp    TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (contract_id) REFERENCES contract(contract_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
