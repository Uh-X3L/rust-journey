# Rustlings Exercises Wiki

This wiki contains an overview of some Rustlings exercises, including code snippets, detailed explanations, and theory. 


## 1. Functions: `call_me`

**Initial Code**

```rust
// TODO: Add some function with the name `call_me` without arguments or a return value.

fn call_me() {
    // function body can be empty
}

fn main() {
    call_me(); // Don't change this line
}
```

**Explanation**  
- **Definition**: Functions in Rust are declared with `fn`.  
- **No Arguments**: If a function takes no arguments, just use empty parentheses `()`.  
- **No Return Value**: Rust functions without return types effectively return the unit type `()`.  

---

## 2. Functions: `bigger`

**Initial Code**

```rust
fn bigger(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

fn main() {
    // ...
}
```

**Explanation**  
- **Comparison**: An `if` expression in Rust can directly return a value.  
- **Condition**: `if a >= b { a } else { b }` picks whichever is larger (or either if equal).  
- **Expression vs. Statement**: In Rust, `if` is an expression, so you can directly put it in the return position.

---

## 3. Move Semantics: `move_semantics2`

### Original Issue

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec;
    new_vec.push(88);
    new_vec
}

#[test]
fn move_semantics2() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    // error[E0382]: borrow of moved value: `vec0`
    assert_eq!(vec0, [22, 44, 66]);
    assert_eq!(vec1, [22, 44, 66, 88]);
}
```

In the above code, `vec0` is moved into the `fill_vec` function because `fill_vec` takes ownership of the argument. Once ownership moves, `vec0` is no longer valid in the caller’s context.

### Fix (Borrowing)

```rust
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.push(88);
    new_vec
}

#[test]
fn move_semantics2() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(&vec0);

    assert_eq!(vec0, [22, 44, 66]);
    assert_eq!(vec1, [22, 44, 66, 88]);
}
```

**Detailed Explanation**  
1. **Function Signature**:  
   - `&Vec<i32>` indicates a *reference* to a vector. Ownership is not transferred.  
2. **Cloning**:  
   - `vec.clone()` creates a new `Vec<i32>` inside the function, leaving the original vector untouched.  
3. **Returning the New Vector**:  
   - `new_vec` is returned, and its ownership moves to the caller as `vec1`.  
4. **Result**:  
   - `vec0` remains usable because only a reference was borrowed.  

### Rust’s Core Handling
- **Ownership vs. Borrowing**: Passing by reference (`&`) keeps the original owner valid. Passing by value moves ownership.  
- **Clone**: Creates a separate copy. This allows safe modifications without affecting the original data.  
- **Return Moves**: When returning a new vector, that return value is a completely new owner in the caller’s scope.

---
**explain more in detail the life cycle of something borrowed in this example**
When `fill_vec(&vec0)` is called, an immutable reference to `vec0` is created just before entering `fill_vec`. This reference is valid only during the execution of `fill_vec`. Inside the function, Rust knows that `vec` points to `vec0`’s data, which stays alive so long as this reference exists. Once the function returns, the reference itself no longer exists, but `vec0` is still owned by the caller. Because only an immutable reference was borrowed, `vec0` remains valid for use afterward. Meanwhile, the newly cloned and modified vector is returned by value as `vec1`, transferring ownership of that new vector from `fill_vec` to the caller.
---
**why inside functions it needs to be mut, but not outside??**
In Rust, function parameters are treated as new local bindings that default to being immutable. Declaring a function parameter as mut (e.g., mut vec: Vec<i32>) tells the compiler that you plan to mutate that local value. Outside the function, you don’t need mut if you’re not mutating the variable in that scope. In this example:

• Inside the function:

mut vec means “this local binding is allowed to be modified,” so vec.push(88) compiles.
• Outside the function (the caller):
let vec0 = vec![22, 44, 66] is never mutated in main or the test, so it doesn’t need to be declared mut there.
Essentially, each scope has its own rules about whether a variable is considered mutable. The function’s parameter is a fresh binding to the passed-in data, and that binding must be marked mut to allow mutation.

---

## 📘 Part 2 – Rust Book Theory: Ownership & Borrowing

### ✅ Read:
- Chapter 4.1 – What is Ownership?
- Chapter 4.2 – References and Borrowing

### 🔍 Key Concepts

#### 🔑 Ownership
Rust enforces memory safety through a system called ownership. Each value in Rust has a variable that is its "owner." When the owner goes out of scope, the value is dropped (freed from memory).

```rust
{
    let s = String::from("hello");
    // s is valid here
} // s is dropped here
```

#### → Moves
When you assign a variable to another, ownership is moved.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is now invalid, ownership moved to s2
```

Rust does this to prevent double-free errors.

#### ♻️ Borrowing (`&T`)
Instead of transferring ownership, you can "borrow" a value by referencing it:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This allows read-only access, and the original owner keeps ownership.

#### ✅ Mutable Borrowing (`&mut T`)
Allows a function or scope to change the borrowed value:

```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

You must declare the original variable as `mut`, and only one `&mut` reference can exist at a time.

#### ⚠️ Borrowing Rules
1. You can have **either**:
   - One mutable reference
   - Or multiple immutable references
2. References must always be valid
3. Mutable and immutable references **cannot coexist**

This protects you at compile time from data races and undefined behavior.

---

### 📖 Ownership Model Summary

| Topic              | Insight                                                                 |
|-------------------|-------------------------------------------------------------------------|
| Ownership          | Each variable owns its memory until it’s moved or dropped               |
| Moves              | Assigning one variable to another moves ownership                      |
| Borrowing          | `&` allows a reference (read-only by default)                           |
| Mutable Borrowing  | `&mut` allows changing the value                                         |
| Borrowing Rules    | Only 1 mutable OR multiple immutable references at a time               |

