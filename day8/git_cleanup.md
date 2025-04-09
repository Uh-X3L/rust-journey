# 🧹 Day 8 – CLI Tool, Git Cleanup & Author Fix

## ✅ Rust Learning Progress

Today I completed:

- 📦 Move semantics exercises (1–5)
- 🧠 Deepened understanding of references, ownership, and mutation
- 🛠️ Built a fully working CLI tool `contract-cli`:
  - Used `clap` to parse commands like `deposit`, `withdraw`, `status`, `history`
  - Used `rusqlite` to persist balance and transaction data
  - Modeled contract logic with safe `&mut self` methods

## 🧼 Git History Fix (Commit Author Attribution)

Realized my WSL user wasn't configured correctly for GitHub, so my commits weren’t showing up in the contribution graph.

### Fixed by:

```bash
git config --global user.name "My Name"
git config --global user.email "my@email.com"
```

> Now future commits are attributed properly and show in GitHub history.

Optionally, rewrote past commits using `git rebase -i` or `git filter-branch` to update author info and force push.

---

## 🚫 Git Hygiene for Small Projects

Noticed extra junk was being pushed (e.g. `target/`, `.db`, `.vscode/`).

### Created a central `.gitignore`:

```gitignore
# === Rust ===
/target
Cargo.lock
**/*.rs.bk

# === SQLite ===
*.db
*.sqlite

# === IDEs ===
.vscode/
.idea/

# === OS files ===
.DS_Store
Thumbs.db

# === Logs ===
*.log
```

### Applied to all `small_projects/` using:

```bash
for dir in */ ; do cp ../template/.gitignore "$dir"; done

find . -type d -name target -exec git rm -r --cached {} \;
find . -type f -name "*.db" -exec git rm --cached {} \;
find . -type d -name .vscode -exec git rm -r --cached {} \;

git add .
git commit -m "chore: cleanup small_projects, apply .gitignore to all tools"
git push
```

---

## 🧠 Lessons Learned

- Use `.gitignore` from Day 1 of any project
- Commit identity matters for public GitHub contributions
- Reusable project scaffolding (with correct `.gitignore`) saves future cleanup

---

## 🔜 Next

- Start Day 9: Structuring & Testing `contract-cli`
- Add user prompts with `inquire` or `dialoguer`
- Push reusable Rust CLI template to GitHub

```
