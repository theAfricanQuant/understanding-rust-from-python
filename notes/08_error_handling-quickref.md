# Lesson 08 — Error Handling — Quick Reference

**Completed:** 21st June, 2026 · **Status:** done

Verbose explainer: `08_error_handling.qmd`

---

## The big mental shift

| Concept | Python | Rust |
|---|---|---|
| Error mechanism | exceptions (throw) | return values (Result) |
| Signatures | can't see what exceptions may be raised | `Result<T, E>` tells you it can fail |
| Caller obligation | optional (forgotten try/except crashes at runtime) | **mandatory** (compiler error if not handled) |
| Unrecoverable | `raise` | `panic!` |
| Catch-all | `except Exception` | `match` (exhaustive) |
| Early propagate | automatic | `?` operator (explicit opt-in) |

---

## Result<T, E> — it's just an enum

```rust
enum Result<T, E> {
    Ok(T),   // success value
    Err(E),  // error value
}
```

Creating:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

---

## Match on Result

```rust
match divide(10.0, 2.0) {
    Ok(v) => println!("{v}"),
    Err(e) => println!("Error: {e}"),
}
```

---

## unwrap() and expect() — escape hatches

| Method | On `Ok(v)` | On `Err(e)` |
|---|---|---|
| `.unwrap()` | returns `v` | panics with default message |
| `.expect("msg")` | returns `v` | panics with `"msg: {e}"` |

```rust
let x = Ok(42).unwrap();           // 42
let x = Ok(42).expect("always ok"); // 42
// Err("boom").unwrap();            // PANICS
// Err("boom").expect("oops");      // PANICS: "oops: boom"
```

**Use sparingly.** `match` or `?` is preferred in production code.

---

## Graceful fallbacks

```rust
let err: Result<i32, &str> = Err("oops");

err.unwrap_or(0);                    // 0 (immediate fallback)
err.unwrap_or_else(|e| e.len() as i32); // 4 (lazy fallback)
err.unwrap_or_default();             // 0 (if E: Default)

// Convert Option to Result:
let opt: Option<i32> = None;
opt.ok_or("missing value");          // Err("missing value")
```

---

## The ? operator — early return on error

```rust
fn read_config(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // ← returns Err if fails
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // ← returns Err if fails
    Ok(content.trim().to_string())       // success
}
```

`?` is syntactic sugar for:

```rust
let mut file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return Err(e),
};
```

**Requirements:** Can only be used in functions that return `Result` (or `Option`). The error type must match or be convertible via `From`.

---

## Custom error types

```rust
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO: {e}"),
            MyError::Parse(e) => write!(f, "Parse: {e}"),
            MyError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for MyError {}

// ? auto-converts with From impls:
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self { MyError::Io(e) }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self { MyError::Parse(e) }
}
```

Now `?` works across functions returning `Result<T, MyError>` even when inner functions return different error types.

---

## Error propagation

```rust
// ? chains fallible calls:
fn load_and_parse(path: &str) -> Result<i32, MyError> {
    let content = fs::read_to_string(path)?;   // io::Error → MyError
    let num: i32 = content.trim().parse()?;     // ParseIntError → MyError
    Ok(num * 2)
}
```

### map_err() — manual conversion (no From impl needed)

```rust
let result: Result<String, MyError> =
    fs::read_to_string(path)
        .map_err(|e| MyError::Io(e))?;
```

---

## main() returning Result

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    println!("{content}");
    Ok(())
}
```

- `Ok(())` = no error → exit code 0
- `Err(e)` = error printed → exit code 1

`Box<dyn Error>` is a type-erased error — any error type works.

---

## panic! vs Result

| Use `panic!` when... | Use `Result` when... |
|---|---|
| It's a programming bug (out of bounds) | It's an expected failure (file not found) |
| The program is in an unrecoverable state | The caller can reasonably recover |
| Assertion failures (`assert!`) | User input that doesn't parse |
| Tests (test failures use `panic!`) | Network timeouts, permission errors |

**Python heuristic:** If you'd use `raise AssertionError` → `panic!`. If you'd use `raise ValueError`/`IOError` → `Result`.

---

## Common methods cheat sheet

| Method | What it does |
|---|---|
| `r.unwrap()` | `Ok(v)` → `v`; `Err` → panic! |
| `r.expect("msg")` | Like unwrap with custom message |
| `r.unwrap_or(default)` | `Ok(v)` → `v`; `Err` → `default` |
| `r.unwrap_or_else(\|e\| ...)` | Lazy fallback |
| `r.unwrap_or_default()` | Fallback to `Default::default()` |
| `r.map_err(\|e\| ...)` | Transform error, keep value |
| `r.or_else(\|e\| ...)` | Try alternative on error |
| `r.ok()` | Convert `Result<T,E>` to `Option<T>` (drops error) |
| `opt.ok_or(e)` | Convert `Option<T>` to `Result<T, E>` |
| `r.is_ok()` / `r.is_err()` | Boolean check |
| `r?` | Propagate error (early return) |

---

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/08_error_handling.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 08_error_handling
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/08_error_handling.rs -o /tmp/08_error_handling && /tmp/08_error_handling
```

---

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `? couldn't convert the error to X` | Error type mismatch with `?` | Add `From` impl or use `map_err` |
| `the ? operator can only be used in a function that returns Result` | Used `?` in `fn main()` or non-Result fn | Change return type to `Result`, or use `match` instead |
| `mismatched types: expected Result<_,_>, found _` | Return type doesn't match function signature | Check your `Ok(...)` and `Err(...)` types |
| `the trait bound E: std::fmt::Display is not satisfied` | Custom error doesn't impl Display | Add `impl std::fmt::Display for MyError` |
| `use of moved value` after `?` | You used `?` on a moved Result | Restructure: use `map_err` or store before `?` |

---

## Key mental models

- **Result is just an enum** — nothing magic. `match` works exactly like it does with any enum.
- **? is syntactic sugar** — it's a `match` with `return Err(e)` on the error arm.
- **Error types must be convertible** — implement `From` for smooth `?` chaining across error types.
- **Return type documents failure modes** — Python can't; Rust can (and enforces).
- **Avoid unwrap in production** — `match` or `?` with proper error handling.
- **panic! is for bugs, Result is for expected failures** — same mental model as AssertionError vs ValueError in Python.
