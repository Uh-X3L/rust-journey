# 📒 Day 11 – Notes & Recap: Building a Rust Migration System

## 🧱 Objective

The goal of Day 11 was to build a proper **migration system** in Rust that supports both `.sql` and `.rs` files, tracks their application status, and integrates with our contract CLI project.

---

## ✅ What Was Implemented

### 1. 🗃️ Migration Engine
- A reusable function `apply(conn: &Connection)` that:
  - Scans the `migrations/` directory
  - Applies any `.sql` or `.rs` files not yet logged
  - Tracks applied migrations in a `migrations` table

### 2. 📋 Migrations Table
```sql
CREATE TABLE IF NOT EXISTS migrations (
    filename TEXT PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'pending',
    error TEXT,
    duration INTEGER
);
```
- Captures detailed status for observability:
  - ✅ Success
  - ❌ Failure (plus error details)
  - ⏱ Execution time

---

## 🦀 Rust Migration Support

### Example: `m_20240414_002_data_transform.rs`
- Added a Rust-based migration script that:
  - Reads data from `old_contract` and `old_transactions`
  - Hashes owner names into `contract_id`
  - Migrates and upserts into the new schema

### Run Logic
```rust
match filename.strip_suffix(".rs") {
    Some("m_20240414_002_data_transform") => crate::db::migrations::m_20240414_002_data_transform::run(conn),
    _ => Err(anyhow::anyhow!("Unknown Rust migration")),
}
```

---

## 📁 Folder Structure

```
migrations/
├── m_20240413_001_init_schema.sql
├── m_20240414_002_data_transform.rs
├── m_20240414_003_cleanup.sql
```

---

## 🚀 Usage

```bash
# Call from main.rs or manually:
db::migrations::apply(&conn)?;
```

---

## 🛡️ Best Practices Used

- ✅ Error handling with `anyhow::Result`
- ✅ Tracked migration history to avoid duplicate runs
- ✅ Captured duration and logged errors
- ✅ Isolated logic for data transformations in Rust
- ✅ Committed to a timestamp-based naming convention (e.g. `m_YYYYMMDD_NNN_description.rs`)

---

## 📌 Why This Matters

- Reproducible schema changes
- Easy rollback/upgrade paths
- Foundation for real-world software release pipelines
- Reinforces modular, testable design in Rust

---

## 🧪 Next Steps

- Add automated tests for the migration runner (optional)
- Add CLI command: `cargo run -- migrate`
- Continue toward Day 12 (traits, abstractions, and early cloud prep)

---

## 🙌 Reflection

Day 11 was a strong move toward **professional backend design**.  
By combining Rust's safety guarantees with solid database practices, you're building tools that are:

- ⚙️ Robust
- 💡 Maintainable
- 🌍 Ready for integration with cloud services and real-world data flows

