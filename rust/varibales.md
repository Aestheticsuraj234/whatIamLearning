# Rust Variables and Data Types

Rust is a statically typed language, meaning that types are checked at compile time. However, it also features type inference, so you don't always need to annotate types explicitly. Let's explore variables and data types in Rust.

---

## 1. Variables in Rust

### Immutable and Mutable Variables
By default, Rust variables are **immutable**, meaning their values cannot be changed after assignment. You can make them **mutable** using `mut`.

#### Example:
```rust
fn main() {
    let x = 5; // Immutable variable
    println!("x = {}", x);

    let mut y = 10; // Mutable variable
    println!("y = {}", y);
    y = 20;
    println!("Updated y = {}", y);
}
```

### Shadowing
Rust allows variable shadowing, where you declare a variable with the same name but a different type or value.

#### Example:
```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing x
    let x = x * 2;
    println!("x = {}", x); // Output: 12
}
```

---

## 2. Data Types in Rust
Rust has two major categories of data types:
1. **Scalar Types**
2. **Compound Types**

### 2.1 Scalar Types
Scalar types represent a single value. Rust has four primary scalar types:

#### **1. Integer Types**
Rust supports signed and unsigned integers of various sizes.

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`   | `u8`     |
| 16-bit  | `i16`  | `u16`    |
| 32-bit  | `i32`  | `u32`    |
| 64-bit  | `i64`  | `u64`    |
| 128-bit | `i128` | `u128`   |
| arch    | `isize` | `usize` |

**Example:**
```rust
fn main() {
    let a: i32 = -10;
    let b: u32 = 20;
    println!("Signed: {}, Unsigned: {}", a, b);
}
```

#### **2. Floating-Point Types**
Rust has `f32` and `f64` for floating-point numbers.

**Example:**
```rust
fn main() {
    let pi: f64 = 3.1415;
    let e: f32 = 2.718;
    println!("pi: {}, e: {}", pi, e);
}
```

#### **3. Boolean Type**
Rust has a `bool` type with `true` or `false` values.

**Example:**
```rust
fn main() {
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
}
```

#### **4. Character Type**
Rust has a `char` type, which represents a single Unicode character.

**Example:**
```rust
fn main() {
    let letter: char = 'R';
    let emoji: char = '😎';
    println!("Letter: {}, Emoji: {}", letter, emoji);
}
```

---

### 2.2 Compound Types
Compound types group multiple values into one type. Rust has two primary compound types:

#### **1. Tuples**
Tuples group multiple values of different types together.

**Example:**
```rust
fn main() {
    let person: (i32, f64, char) = (30, 5.9, 'M');
    let (age, height, gender) = person;
    println!("Age: {}, Height: {}, Gender: {}", age, height, gender);
}
```

#### **2. Arrays**
Arrays contain multiple values of the same type and have a fixed length.

**Example:**
```rust
fn main() {
    let numbers: [i32; 4] = [10, 20, 30, 40];
    println!("First: {}, Second: {}", numbers[0], numbers[1]);
}
```

---

## Summary
- **Immutable by default**: Use `let mut` for mutability.
- **Shadowing** allows redeclaring variables with the same name.
- **Scalar types**: Integers, Floats, Booleans, Characters.
- **Compound types**: Tuples and Arrays.

