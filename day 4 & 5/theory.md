# 📘 Rust Fundamentals: Functions, Control Flow, Ownership, Enums & Structs

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

### 🔚 Return Values
- The last line without a semicolon is returned.
- You can also use `return`, but implicit return is more idiomatic.

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn double_explicit(x: i32) -> i32 {
    return x * 2;
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

---

## 🧪 Quick Examples

```rust
fn square(n: i32) -> i32 {
    n * n
}

fn choose_number(condition: bool) -> i32 {
    if condition { 1 } else { 2 }
}
```

---

## 📘 Ownership & Borrowing (Ch. 4.1, 4.2)

### ✅ Concepts
- Each value has one owner
- Values are moved by default when assigned
- Borrowing lets you use a reference without taking ownership

### 🔍 Borrowing Rules
- One `&mut` or many `&` at a time, never both

```rust
let s = String::from("hello");
let r1 = &s; // allowed
let r2 = &s; // also allowed
// let r3 = &mut s; // not allowed if above exist
```

---

## 🔹 Advanced Enums with `impl`

```rust
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

impl Command {
    fn run(&self) {
        match self {
            Command::Quit => println!("Exiting..."),
            Command::Echo(msg) => println!("Echo: {}", msg),
            Command::Move { x, y } => println!("Move to ({}, {})", x, y),
        }
    }
}
```

---

## 🔹 Match Expressions (Ch. 6.2)

```rust
fn print_number(num: Option<i32>) {
    match num {
        Some(n) if n > 0 => println!("Positive: {}", n),
        Some(n) => println!("Zero or Negative: {}", n),
        None => println!("No number provided"),
    }
}
```

### Pattern Matching With Enums
```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle { width, height } => width * height,
    }
}
```

---

## 🔹 Result Type for Error Handling (Ch. 9.2)

### `Result<T, E>`
Rust's standard enum for recoverable errors:

```rust
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}

match divide(10, 2) {
    Ok(val) => println!("Result: {}", val),
    Err(e) => println!("Error: {}", e),
}
```

- Use `match`, `unwrap`, `expect`, or `?` to work with `Result`

---

## 📘 Structs & Methods (Ch. 5.1–5.3)

### ✅ Basic Struct
```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};
```

### ✅ Tuple Struct & Unit Struct
```rust
struct Color(i32, i32, i32);
struct Marker;
```

---

### ✅ Struct with Methods (Ch. 5.3)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect1.area());
```

### 🧠 Why This Matters
- `impl` gives behavior to your data
- You can chain methods or check conditions on real-world objects
- Great for modeling app logic cleanly and expressively
