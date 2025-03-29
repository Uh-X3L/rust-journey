# 📘 Rust Fundamentals: Functions & Control Flow

## 🔹 1. Functions in Rust (Chapter 3.3)

Functions in Rust are defined using the `fn` keyword and follow this structure:

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body
}
```

### ✅ Key Concepts

#### 🧠 Function Signatures
- Parameter types must always be declared (Rust has no type inference for parameters).
- Return types use the `->` syntax.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

#### ↻ Function Calls
```rust
fn main() {
    print_number(7);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}
```

---

### 🔚 Return Values
- The last line without a semicolon is returned.
- You can also use `return`, but implicit return is more idiomatic.

```rust
fn double(x: i32) -> i32 {
    x * 2  // implicit return
}

fn double_explicit(x: i32) -> i32 {
    return x * 2; // explicit return
}
```

---

## 🔹 2. Control Flow (Chapter 3.5)

Rust supports standard control flow: `if`, `else`, `loop`, `while`, `for`

### ✅ Key Concepts

#### 🧠 `if` as an Expression
- `if` returns a value.
- Both branches must return the same type.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The value is: {}", number);
}
```

#### ❌ No parentheses in `if`
```rust
if x > 10 {
    println!("Big number");
}
```
✅ Blocks must always use `{}` even for single-line conditions.

---

## 🧪 Quick Examples

### Example 1: Function Returning a Value
```rust
fn square(n: i32) -> i32 {
    n * n
}
```

### Example 2: `if` Returning a Value
```rust
fn choose_number(condition: bool) -> i32 {
    if condition {
        1
    } else {
        2
    }
}
```

---

## ✍️ Summary Cheat Sheet

| Concept            | Rust Style                              |
|--------------------|------------------------------------------|
| Function           | `fn add(x: i32, y: i32) -> i32 { x + y }` |
| Explicit return    | `return x + y;`                          |
| Implicit return    | No `;` at end of last line               |
| If as expression   | `let val = if cond { 1 } else { 2 };`    |
| Type requirements  | Branches in `if` must return same type   |
| No parens in `if`  | `if x > 5 { ... }` ✅                     |


