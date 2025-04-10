-- Migration to new schema with hashed TEXT contract IDs
ALTER TABLE contract RENAME TO old_contract;
ALTER TABLE transactions RENAME TO old_transactions;

PRAGMA foreign_keys = ON;

CREATE TABLE contracts (
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

    FOREIGN KEY (contract_id) REFERENCES contracts(contract_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
