# 🧾 contract-cli

> A simple, testable smart contract simulation CLI — built in Rust with SQLite and `clap`.

This CLI simulates wallet-like smart contract behaviors: deposits, withdrawals, balance checks, and transaction logs — all backed by a lightweight SQLite database.  

Built as part of a Rust blockchain engineering journey, it focuses on clean architecture, testability, and data safety.

---

## ✨ Features

- 🔐 Owner identity is hashed using SHA-512 (64-bit collision-safe) for secure and consistent IDs  
- 💰 Deposit and withdraw functionality (with safe DB checks)  
- 🧾 View recent transaction history (limited to last 5)  
- 🗃️ SQLite database storage for state persistence  
- 🔑 Safe SQL using parameterized queries (`params![]`) to prevent injection  
- 📦 Modern CLI interface using [`clap`](https://crates.io/crates/clap)  
- 🧪 In-memory unit tests for full logic coverage  

---

## 📦 Installation

### Prerequisites

- Rust (via [rustup.rs](https://rustup.rs))  
- SQLite development libraries:

```bash
# Linux / WSL (Debian-based)
sudo apt update
sudo apt install libsqlite3-dev
```

---

### Clone & Build

```bash
git clone https://github.com/Uh-X3L/rust-journey.git
cd rust-journey/z_small_projects/contract-cli
cargo build
```

---

## 🚀 Usage

```bash
cargo run -- --owner alice status
cargo run -- --owner alice deposit --amount 200
cargo run -- --owner alice withdraw --amount 50
cargo run -- --owner alice history
```

### Example Output

```bash
$ cargo run -- --owner alice status
👤 Owner: alice
💰 Balance: 150

$ cargo run -- --owner alice history
📜 Last 5 transactions for alice:
123 | deposit: 200
124 | withdraw: 50
```

---

## 🧪 Running Tests

This project includes unit tests for deposit, withdrawal, owner isolation, and transaction logic.

```bash
cargo test
```

To display test output (e.g. println!):

```bash
cargo test -- --nocapture
```

---

## 📁 Project Structure

```
contract-cli/
├── src/
│   ├── main.rs           # CLI & command routing
│   ├── contract.rs       # Core smart contract logic
│   ├── db/
│   │   ├── mod.rs        # DB connection and setup
│   │   └── migrations.rs # DB schema & future upgrades
│   └── utils/
│       └── hash.rs       # Owner hashing utility (SHA-512)
├── Cargo.toml
└── README.md
```

---

## 🔐 Security Notes

Rust encourages safe practices by design.  
All SQL operations use parameterized queries to avoid injection:

```rust
conn.prepare("SELECT ... WHERE id = ?1")?.query(params![id])?;
```

Using this approach:
- ✅ protects from SQL injection
- ✅ ensures consistent type handling
- ✅ improves query reusability and performance

---

## 👨‍🔬 Learning Highlights

- ✅ Modular Rust design (submodules for utils/db)  
- ✅ Secure hashing with `sha2` for identity handling  
- ✅ SQLite via `rusqlite` with safe `Result<T>`-based error handling  
- ✅ CLI structure and command parsing with `clap`  
- ✅ Hands-on practice with unit tests, schema integrity, and constraint checks  

---

## 🛣️ Roadmap

- [x] Multi-user contract support via hashed IDs  
- [x] Enforced constraint-safe logging (no negative amounts)  
- [ ] Export history to CSV/JSON  
- [ ] CLI auto-complete / interactive mode  
- [ ] Optional web interface via `actix-web`  

---

## 📚 License

MIT — free to use and build upon.

---

## 🤝 Credits

Built with ❤️ during my Rust + Blockchain engineering transition.  
Follow the journey: [github.com/Uh-X3L](https://github.com/Uh-X3L)
