/// Lesson 08 — Error Handling
/// Rust side: panic! vs Result<T,E>, unwrap/expect, ? operator,
/// error propagation, custom error types, and main() returning Result.
///
/// Biggest mental shift from Python:
/// Python → errors are THROWN (exceptions interrupt control flow)
/// Rust   → errors are RETURNED (Result<T,E> is just a value you handle)

use std::fs::{self, File};
use std::io::{self, Read};
use std::num::ParseIntError;

/// ---- SECTION 1: panic! — unrecoverable errors ----
/// Python: raise ValueError("bad!") — exceptions everywhere
/// Rust:   panic!("bad!") — only for BUGS, not expected errors
///         For recoverable errors, use Result<T, E>

fn demo_panic() {
    // panic!("crash and burn");  // ← uncomment to see it, but it kills the demo

    // You can set RUST_BACKTRACE=1 to see the stack trace
    // Similar to Python's traceback

    println!("  (panic! demo: commented out so we don't stop here)");
    println!("  Try: RUST_BACKTRACE=1 cargo run --bin 08_error_handling");
}

/// ---- SECTION 2: Result<T, E> — the recoverable error enum ----
/// Python: return None or raise
/// Rust:   return Result<T, E> — it's literally an enum!
///         enum Result<T, E> { Ok(T), Err(E) }
///         The compiler FORCES callers to handle both cases.

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn demo_result_enum() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(v) => println!("  10/2 = {v}"),
        Err(e) => println!("  10/2 error: {e}"),
    }

    let result = divide(10.0, 0.0);
    match result {
        Ok(v) => println!("  10/0 = {v}"),
        Err(e) => println!("  10/0 error: {e}"),
    }

    // Key insight: you CANNOT accidentally use the value:
    // println!("{}", result + 1.0);  // ❌ ERROR: cannot add to Result
    // You MUST handle the error case.
}

/// ---- SECTION 3: unwrap() and expect() — the escape hatches ----
/// Quick ways to get the value out of a Result, but they PANIC on Err.
/// Use for quick prototyping, tests, or when failure is logically impossible.

fn demo_unwrap_expect() {
    let ok_result: Result<i32, &str> = Ok(42);
    println!("  unwrap Ok: {}", ok_result.unwrap());

    // let err_result: Result<i32, &str> = Err("boom");
    // err_result.unwrap();  // PANICS: called `Result::unwrap()` on an `Err` value
    // err_result.expect("CPU exploded");  // PANICS: CPU exploded: "boom"

    // expect() is like unwrap() but with a custom message — better for debugging
    let config: Result<i32, &str> = Ok(8080);
    println!(
        "  expect on Ok: {}",
        config.expect("config should always be valid here")
    );

    // "Real" Rust code uses match or ? instead.
    // unwrap/expect are code smells in production.
}

/// ---- SECTION 4: The ? operator — early return on error ----
/// Python: exceptions bubble up automatically
/// Rust:   ? operator returns the error immediately (syntactic sugar for match)
///         let value = fallible_fn()?;
///         // If fallible_fn() returns Err(e), this function returns Err(e) NOW.
///         // If Ok(v), v is unwrapped and execution continues.
///
/// Only works inside functions that return Result (or Option).

fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ← if File::open fails, return Err immediately
    let mut content = String::new();
    file.read_to_string(&mut content)?; // ← if read_to_string fails, return Err
    Ok(content) // success
}

fn demo_question_mark() {
    match read_file_content("Cargo.toml") {
        Ok(content) => println!("  read Cargo.toml: {} bytes", content.len()),
        Err(e) => println!("  error reading file: {e}"),
    }

    match read_file_content("/nonexistent/file.txt") {
        Ok(content) => println!("  {content}"),
        Err(e) => println!("  expected error: {e}"),
    }

    // Without ?, you'd write:
    // let file = match File::open(path) {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };
    // ? is syntactic sugar that eliminated boilerplate!
}

/// ---- SECTION 5: Error propagation — passing errors upward ----
/// When a function calls other fallible functions, you chain them with ?.
/// The error type must be the same, or convertible (see §7).

fn read_username_from_file() -> Result<String, io::Error> {
    // The idiomatic chain:
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Even shorter (stdlib convenience):
    // fs::read_to_string("hello.txt")
}

fn demo_propagation() {
    match read_username_from_file() {
        Ok(name) => println!("  username: {name}"),
        Err(e) => println!("  couldn't read username: {e}"),
    }
}

/// ---- SECTION 6: Unwrap strategies for Option and Result ----
/// Methods that handle errors gracefully:

fn demo_unwrap_strategies() {
    // Result methods:
    let ok: Result<i32, &str> = Ok(7);
    let err: Result<i32, &str> = Err("oops");

    println!("  unwrap_or:    {}", err.unwrap_or(0)); // fallback: 0
    println!("  unwrap_or_else: {}", err.unwrap_or_else(|e| e.len() as i32)); // lazy fallback: 4
    println!("  ok.or_else:   {:?}", ok.or_else(|_| Err("alternative"))); // Ok(7)
    println!("  err.or_else:  {:?}", err.or_else(|e| Err(format!("wrapped: {e}")))); // Err(...)

    // Option methods:
    let _some_val: Option<i32> = Some(5);
    let none_val: Option<i32> = None;
    println!("  option unwrap_or: {}", none_val.unwrap_or(0));
    println!("  option ok_or:     {:?}", none_val.ok_or("missing value"));

    // ok_or converts Option to Result:
    // None => Err("missing value"), Some(v) => Ok(v)
}

/// ---- SECTION 7: Custom error types with enum ----
/// Python: subclass Exception
/// Rust:   define an enum, implement Display and Error traits
///         Then your functions return Result<T, MyError>

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

// Implement Display so errors can be printed
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {e}"),
            MyError::Parse(e) => write!(f, "Parse error: {e}"),
            MyError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

// Implement Error (optional but recommended for interoperability)
impl std::error::Error for MyError {}

// Implement From conversions so ? can auto-convert
impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn read_and_parse(path: &str) -> Result<i32, MyError> {
    // ? works with io::Error because we implemented From<io::Error> for MyError
    let content = fs::read_to_string(path)?;

    // ? works with ParseIntError because we implemented From<ParseIntError> for MyError
    let num: i32 = content.trim().parse()?;

    Ok(num * 2)
}

fn demo_custom_error() {
    // Create a file with a number for this demo
    let _ = fs::write("/tmp/test_number.txt", "42");

    match read_and_parse("/tmp/test_number.txt") {
        Ok(n) => println!("  result: {n}"),
        Err(e) => println!("  error: {e}"),
    }

    match read_and_parse("/nonexistent/file.txt") {
        Ok(n) => println!("  {n}"),
        Err(e) => println!("  expected error: {e}"),
    }

    // Clean up
    let _ = fs::remove_file("/tmp/test_number.txt");
}

/// ---- SECTION 8: map_err — manual error conversion ----
/// When you can't use From (e.g., third-party types or one-off conversions),
/// use .map_err() to convert the error inline.

fn demo_map_err() {
    let result: Result<i32, &str> = Ok(42);

    // Convert &str error to String error
    let converted: Result<i32, String> =
        result.map_err(|e| format!("converted: {e}"));
    println!("  map_err: {converted:?}");

    // map_err on Err:
    let err_result: Result<i32, &str> = Err("raw error");
    let wrapped: Result<i32, String> =
        err_result.map_err(|e| format!("wrapped: {e}"));
    println!("  wrapped: {wrapped:?}");

    // Practical use: wrapping a std error into your custom error
    let io_result = fs::read_to_string("/tmp/nonexistent");
    let my_result: Result<String, MyError> =
        io_result.map_err(|e| MyError::Io(e));
    println!("  custom: {my_result:?}");

    // Using the Custom variant for one-off errors:
    let custom: Result<(), MyError> = Err(MyError::Custom(String::from("a custom message")));
    println!("  custom variant: {custom:?}");
}

/// ---- SECTION 9: main() returning Result ----
/// You can make main() return a Result and use ? directly!

fn demo_main_result() {
    // Instead of the usual signature: fn main() {
    // You can write: fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Then ? works throughout main:
    //
    // fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     let content = fs::read_to_string("Cargo.toml")?;
    //     println!("{content}");
    //     Ok(())
    // }
    //
    // If main returns Err, the program prints the error and exits with code 1.
    // Ok(()) is the "everything went fine" return.

    println!("  Use fn main() -> Result<(), Box<dyn Error>> for quick ? in main!");
}

/// ---- SECTION 10: panic! vs Result — when to use which ----
/// Mental model for Python devs:
///
/// panic! → for BUGS (programming errors, unrecoverable state)
///   - Out-of-bounds index: v[100]
///   - Assertion failures: assert!(x > 0)
///   - "This should never happen" situations
///   - Tests (test failures use panic!)
///
/// Result → for EXPECTED failures (user error, IO, network, parsing)
///   - File not found
///   - Network timeout
///   - User input that doesn't parse
///   - Anything the CALLER can reasonably recover from

fn demo_panic_vs_result() {
    // Result approach — recoverable:
    let maybe_file = File::open("/nonexistent");
    match maybe_file {
        Ok(_) => println!("  file opened"),
        Err(e) => println!("  file error (Result): {e}"),
    }

    // panic! approach — should NOT be caught in normal code:
    // let v = vec![1, 2, 3];
    // let _bad = v[99];  // PANICS: index out of bounds

    // But you CAN catch panics with catch_unwind (rare, for FFI/testing):
    let result = std::panic::catch_unwind(|| {
        // code that might panic
        let v = vec![1, 2, 3];
        let _ = v.get(99); // safe: returns None instead of panicking
    });
    println!("  catch_unwind: {result:?} (didn't panic because we used .get())");

    // Rule of thumb: if Python would raise an exception for it → use Result.
    //                if Python would be an AssertionError → use panic!.
}

fn main() {
    println!("=== 1. panic! ===");
    demo_panic();
    println!("\n=== 2. Result<T, E> enum ===");
    demo_result_enum();
    println!("\n=== 3. unwrap() and expect() ===");
    demo_unwrap_expect();
    println!("\n=== 4. The ? operator ===");
    demo_question_mark();
    println!("\n=== 5. Error propagation ===");
    demo_propagation();
    println!("\n=== 6. Unwrap strategies ===");
    demo_unwrap_strategies();
    println!("\n=== 7. Custom error types ===");
    demo_custom_error();
    println!("\n=== 8. map_err() ===");
    demo_map_err();
    println!("\n=== 9. main() returning Result ===");
    demo_main_result();
    println!("\n=== 10. panic! vs Result ===");
    demo_panic_vs_result();
}
