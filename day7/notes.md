# Day 7 – Lifetimes & Smart Contract Modeling

## ✅ Rustlings
- [x] lifetimes1
- [x] lifetimes2

## 📘 Book Reading
- [x] Chapter 4.3 – Slices
- [x] Chapter 10.3 – Lifetimes

## 🔍 Key Takeaways
- Lifetimes protect from dangling references
- `'a` ties function return values to input reference lifetimes
- Slices offer zero-copy views of strings and arrays

## 🛠 Smart Contract Design
Modeled a basic `Contract` struct with:
- `owner: String`
- `balance: u64`
- `deposit()` and `withdraw()` with safe mutation and error handling
