## 🧪 Rust Testing – Full Guide for Data Engineering & CLI Projects

---

### ✅ Overview

Rust includes built-in support for testing with zero extra dependencies. Whether you're building a CLI app, contract simulator, or data pipeline, tests help you catch bugs early and enforce correct behavior.

This guide covers:

- 📘 Core syntax from the Rust Book
- 🧠 Best practices and organization
- 🛠️ Templates and real examples
- 📊 Data-focused use cases (SQLite, CSV)
- 🧪 CLI testing with `assert_cmd`

---

## 📘 Core Concepts (Rust Book Ch. 11.1 & 11.2)

### 🔹 Marking and Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
```

- `#[test]`: tells Rust this is a test
- `#[cfg(test)]`: compiles the module **only** for testing
- `cargo test`: runs all tests in the project

---

### 🔹 Assertion Macros

| Macro              | Description                                  |
|--------------------|----------------------------------------------|
| `assert!(cond)`    | Asserts condition is `true`                  |
| `assert_eq!(a, b)` | Passes if `a == b`                           |
| `assert_ne!(a, b)` | Passes if `a != b`                           |
| `#[should_panic]`  | Test must panic to pass                      |

---

### 🔹 Running Tests

```bash
cargo test               # Run all tests
cargo test test_name     # Run one test
cargo test -- --nocapture # Show output (e.g. println!)
cargo test -- --ignored   # Run only #[ignore] tests
```

---

## 🧱 Best Practices

### 1. Use in-memory or temporary resources
- For SQLite: `Connection::open_in_memory()`
- For files: use the `tempfile` crate

### 2. Isolate tests
- Don’t rely on global state
- Use helper functions to avoid duplication

### 3. Keep them small
- Each test should cover one logical behavior

### 4. Use `#[ignore]` for slow or unstable tests

---

## 🧪 Template – SQLite Testing in CLI

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup() -> (Connection, Contract) {
        let conn = Connection::open_in_memory().unwrap();
        let contract = Contract::load_or_create(&conn, 1, "alice").unwrap();
        (conn, contract)
    }

    #[test]
    fn test_deposit() {
        let (conn, mut c) = setup();
        c.deposit(&conn, 100).unwrap();
        assert_eq!(c.balance, 100);
    }

    #[test]
    fn test_withdraw_empty() {
        let (conn, mut c) = setup();
        let result = c.withdraw(&conn, 50);
        assert!(result.is_ok());
        assert_eq!(c.balance, 0);
    }
}
```

---

## 📊 Data Engineering Use Cases

### ✅ 1. CSV Parsing

```rust
#[test]
fn test_csv_to_struct() {
    let data = "id,name\n1,Alice\n2,Bob";
    let records = parse_csv(data);
    assert_eq!(records.len(), 2);
    assert_eq!(records[1].name, "Bob");
}
```

---

### ✅ 2. SQL Query Verification

```rust
#[test]
fn test_insert_and_query() {
    let conn = Connection::open_in_memory().unwrap();
    run_schema(&conn);
    insert_user(&conn, 1, "alice");
    let name = get_user_name(&conn, 1).unwrap();
    assert_eq!(name, "alice");
}
```

---

### ✅ 3. CLI Behavior (using `assert_cmd`)

Add to `Cargo.toml`:
```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
```

Test:

```rust
use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_cli_deposit() {
    let mut cmd = Command::cargo_bin("contract-cli").unwrap();
    cmd.args(["deposit", "--amount", "100"])
       .assert()
       .success()
       .stdout(contains("Deposited 100"));
}
```

---

## 📚 Summary Table

| Feature             | Description                                 |
|---------------------|---------------------------------------------|
| `#[test]`           | Mark test function                          |
| `#[cfg(test)]`      | Compile block only during tests             |
| `assert_eq!()`      | Value equality                              |
| `Connection::open_in_memory()` | SQLite test DB                  |
| `assert_cmd`, `predicates` | CLI integration tests                |
| `#[ignore]`         | Skip test unless explicitly run             |
| `cargo test -- --nocapture` | Show `println!()` output           |

---

## 🧠 Final Thoughts

✅ Rust encourages a test-first mindset:  
- You **compile safer**
- You **build faster**
- You **ship with confidence**

Use tests to lock in your logic — especially when dealing with:
- Mutable state
- External dependencies (e.g. DBs)
- User input or CLI interfaces

