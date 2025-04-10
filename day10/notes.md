# 📘 Day 10 – Contract CLI Refactoring & Multi-User Support

Today’s focus was on **improving robustness, modularity, and security** in the contract CLI project — turning it into a more realistic simulation of a smart contract wallet system.

---

## ✅ Key Improvements

### 🔐 Owner Hashing with SHA-512
- Replaced numeric user IDs with **SHA-512 hashes derived from owner names**
- Only the first 64 bits (8 bytes) of the hash are used as a unique, collision-safe identifier
- Ensures privacy and a scalable, deterministic identifier scheme for multi-user support

> ✅ Function: `hash_owner(owner: &str) -> String`

---

### 🧱 Database Schema Updates

- Renamed `id` to `contract_id` for clarity
- Added constraint-safe schema:
  - Primary key: `contract_id TEXT`
  - `amount` column has a `CHECK(amount >= 0)`
  - `tx_type` has a `CHECK(tx_type IN ('deposit', 'withdraw'))`
  - `timestamp` column added (default: `CURRENT_TIMESTAMP`)
- Foreign key constraint on `contract_id` between transactions and contracts

---

### 🔄 Withdrawal Logic Fix

Before:
```rust
self.log_transaction(conn, -(amount as i64), "withdraw")
```

Now:
```rust
self.log_transaction(conn, amount as i64, "withdraw")
```

> Transactions are logged with **positive amounts only** — the type (`withdraw`) defines the intent.  
> Keeps DB consistent with `CHECK(amount >= 0)`.

---

### 🧪 Updated and Added Tests

- All existing tests migrated to use hashed IDs
- ✅ `test_multiple_owners_are_isolated` added
  - Verifies that contract data is fully isolated between users
- ✅ Used `Connection::open_in_memory()` for test DB isolation

---

### 🧰 Modular Structure

```bash
src/
├── main.rs            # CLI parsing and command handling
├── contract.rs        # Core contract logic (load, deposit, withdraw)
├── db/
│   ├── mod.rs         # DB connection
│   └── migrations.rs  # Future migrations & schema bootstrap
└── utils/
    └── hash.rs        # SHA-512 hash-based ID generation
```

---

### 💬 Lessons Learned

- Rust’s pattern of using `params![]` and `?` promotes **safe and readable database code**
- **Test coverage** makes it easier to refactor without fear
- Avoiding **negative values** in logs aligns better with SQL design
- **SHA-512 hashing** is straightforward with the `sha2` crate and offers scalable identity mapping

---

## 📌 Commands Used (Examples)

```bash
cargo run -- --owner alice deposit --amount 200
cargo run -- --owner alice status
cargo run -- --owner alice history
```

---

## 🧠 Next Steps

- [ ] Export transaction history to CSV or JSON
- [ ] Add CLI argument for export format
- [ ] Add optional password field + hash in DB (for authentication simulation)
- [ ] Continue migrating logic into reusable modules
- [ ] Explore `actix-web` or `axum` for REST API version

---

## 🗃️ Git Commit Summary

```bash
feat(contract): enforce clean withdrawal logging + hash-based user ID

- Fixed withdrawal logging to store only positive amounts (aligned with DB CHECK constraint)
- Improved `show_history` output to format withdrawals with negative sign for clarity
- Refactored contract table to use hashed contract_id from owner
- Added `hash_owner()` utility for ID generation using SHA-512 (64-bit)
- Modularized project with `utils::hash` and `db::migrations`
- Added `test_multiple_owners_are_isolated` for user isolation testing
- All tests updated to use hashed IDs and pass successfully
```

---

## 🚀 Resources Used

- [`rusqlite` crate](https://docs.rs/rusqlite)
- [`sha2` crate](https://crates.io/crates/sha2)
- [`clap` CLI crate](https://docs.rs/clap)
- `Connection::open_in_memory()` for testing
