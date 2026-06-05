# Lesson 02 — Variables & Printing

| Python | Rust |
|---|---|
| `name = "Siseng"` | `let name = "Siseng";` |
| `age = 30` | `let age: i32 = 30;` |
| `print(f"{name} is {age}")` | `println!("{name} is {age}");` |
| Mutable by default | **Immutable** by default |
| Types dynamic | Types static (inferred or explicit) |

## Key ideas

### 1. `let` creates a variable
Unlike Python, every statement ends with `;`.

### 2. Types — explicit or inferred
- `let age: i32 = 30;`  — explicit, this is a 32-bit integer
- `let pi = 3.14;`     — Rust infers `f64` (default float)
- `let name = "Siseng";` — Rust infers `&str` (string slice)

### 3. Immutable by default
```rust
let x = 5;
x = 6;  // ERROR: cannot assign twice to immutable variable
```
You'll need `let mut x = 5;` to make it mutable. (Covered in lesson 04.)

### 4. `{}` in println
- `{}` is a placeholder that calls `Display` (works for most types)
- `{name}` is **named capture** — Rust 1.58+ feature, no `f` prefix needed

## Run it
```bash
cd /home/siseng/Documents/programing_languages/Rust/playground
cargo run --bin 02_vars
python3 python/02_vars.py
```
