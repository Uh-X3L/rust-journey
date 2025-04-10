# 🧾 contract-cli

> A simple, testable smart contract simulation CLI — built in Rust with SQLite and `clap`.

This CLI tool simulates core wallet behaviors such as balance management, deposits, withdrawals, and viewing transaction history, all backed by a lightweight SQLite database.

It was created as part of a Rust blockchain engineering learning journey — with a focus on modular design, safe state handling, and practical CLI development.

---

## ✨ Features

- 🔐 Persistent owner-based contract data  
- 💰 Deposit and withdraw balance via CLI  
- 🧾 View last 5 transactions  
- 🗃️ Data persistence using SQLite  
- 📦 Command-line interface via [`clap`](https://crates.io/crates/clap)  
- 🧪 Built-in unit tests with in-memory databases  

---

## 📦 Installation

### Prerequisites

- Rust (via [rustup.rs](https://rustup.rs))  
- SQLite (installed system-wide):

```bash
# Linux / WSL (Debian-based)
sudo apt update
sudo apt install libsqlite3-dev
```

---

### Clone & Build

```bash
git clone https://github.com/Uh-X3L/rust-journey.git
cd rust-journey/small_projects/contract-cli
cargo build
```

---

## 🚀 Usage

```bash
cargo run -- status
cargo run -- deposit 200
cargo run -- withdraw 50
cargo run -- history
```

### Example Output

```bash
$ cargo run -- status
👤 Owner: alice
💰 Balance: 150

$ cargo run -- history
📜 Last 5 transactions:
[Deposit] +200
[Withdraw] -50
```

---

## 🧪 Running Tests

This project includes unit tests for contract logic.

```bash
cargo test
```

To display debug output:

```bash
cargo test -- --nocapture
```

---

## 📁 Project Structure

```
contract-cli/
├── src/
│   ├── main.rs        # CLI setup & entrypoint
│   └── contract.rs    # Core logic
├── Cargo.toml
└── README.md
```

---

## 👨‍🔬 Learning Goals

This project reinforced:

- ✅ Modular Rust design (multiple files/modules)  
- ✅ Safe and testable DB state with SQLite  
- ✅ CLI dev with `clap`  
- ✅ Real-world `Result`, `unwrap()`, `match`  
- ✅ Tests with `#[cfg(test)]` + `Connection::open_in_memory()`  

---

## 🛣️ Roadmap

- [ ] Multi-user support  
- [ ] Export to JSON/CSV  
- [ ] Web UI with `actix-web`  
- [ ] Command auto-complete / help menu  

---

## 📚 License

MIT — use freely and build on it.

---

## 🤝 Credits

Built with ❤️ during my Rust + Blockchain engineering journey.  
More at: [github.com/Uh-X3L](https://github.com/Uh-X3L)
