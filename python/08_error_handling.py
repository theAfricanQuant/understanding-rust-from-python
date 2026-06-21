"""Lesson 08 — Error Handling
Python side: demonstrating exceptions, try/except, and simulating Rust's Result pattern.
Rust's approach is fundamentally different: errors are RETURN VALUES, not thrown exceptions."""

from typing import Optional


# ============================================================
# SECTION 1: panic! vs Python exceptions
# ============================================================
# Python: raise ValueError("bad!")  — an exception UNWINDS the call stack
# Rust:   panic!("bad!")          — also unwinds (or aborts), but only for
#                                  UNRECOVERABLE errors.
#         Rust devs use panic! sparingly. In Python, exceptions are everywhere.

def demo_panic_equivalent():
    """Python: exceptions are the norm. Rust: panic! is for bugs only."""
    try:
        raise ValueError("Something went wrong!")
    except ValueError as e:
        print(f"  Caught: {e}")

    # Python: you raise/catch for control flow. Examples: StopIteration, KeyError.
    # Rust:   you'd NEVER use panic! for control flow.
    #         panic! means "the program is broken, stop now."
    #         For normal errors, Rust uses Result<T, E> — a RETURN VALUE.


# ============================================================
# SECTION 2: Result<T, E> — errors as return values
# ============================================================
# Python: functions raise exceptions on error
# Rust:   functions return Result<T, E> — Ok(T) on success, Err(E) on failure
#         enum Result<T, E> { Ok(T), Err(E) }  <-- it's literally an enum!
#
# Python can simulate this with a union return type, but it's not enforced.

class RustyResult:
    """Simulating Rust's Result enum in Python. Messy and unidiomatic."""
    pass


class Ok_:
    """Rust: Ok(T) — the success variant."""
    def __init__(self, value):
        self.value = value
        self.is_ok = True
        self.is_err = False

    def unwrap(self):
        return self.value


class Err_:
    """Rust: Err(E) — the error variant."""
    def __init__(self, error):
        self.error = error
        self.is_ok = False
        self.is_err = True

    def unwrap(self):
        raise RuntimeError(f"Called unwrap() on an Err: {self.error}")


def divide_rusty(a: float, b: float) -> Ok_ | Err_:
    """Python simulating Rust's Result return pattern."""
    if b == 0:
        return Err_("Division by zero")
    return Ok_(a / b)


def demo_result_pattern():
    """Python: result pattern — you MUST check before using the value."""
    result = divide_rusty(10, 2)
    if result.is_ok:
        print(f"  10/2 = {result.value}")
    else:
        print(f"  10/2 error: {result.error}")

    result = divide_rusty(10, 0)
    if result.is_ok:
        print(f"  10/0 = {result.value}")
    else:
        print(f"  10/0 error: {result.error}")

    # Rust equivalent:
    # let result = divide(10.0, 2.0);
    # match result {
    #     Ok(v) => println!("10/2 = {v}"),
    #     Err(e) => println!("10/2 error: {e}"),
    # }

    # The key insight: Result FORCES the caller to handle the error.
    # In Python, you can forget the try/except and crash at runtime.


# ============================================================
# SECTION 3: unwrap() and expect() — the escape hatches
# ============================================================
# Python: you can ignore errors until runtime crash
# Rust:   opt.unwrap()    — "give me the value or PANIC"
#         opt.expect("msg") — same but with a custom panic message
# These are for when you're SURE it can't fail, or for quick prototyping.

def demo_unwrap_expect():
    """Python: simulating Rust's unwrap and expect."""
    ok_result = Ok_(42)
    print(f"  unwrap Ok: {ok_result.unwrap()}")

    err_result = Err_("boom")
    # err_result.unwrap()  # would raise RuntimeError — like Rust's panic!

    print("  expect on Ok:", Ok_(99).unwrap())

    # In real Python, you might write:
    # value = result.unwrap()  # crash if Err_ — like Rust's unwrap()

    # Python's analogue of Rust's expect("msg"):
    # try:
    #     value = result.unwrap()
    # except RuntimeError as e:
    #     raise RuntimeError("expected valid result: {e}")

    # Best practice: avoid unwrap/expect in production.
    # Use match (Rust) or try/except (Python) instead.


# ============================================================
# SECTION 4: The ? operator — early return on error
# ============================================================
# Rust:   let value = fallible_fn()?;
#         If fallible_fn() returns Err(e), the ? returns Err(e) immediately.
#         If Ok(v), it unwraps and continues. Only works in fn -> Result.
#
# Python: This doesn't exist natively. The closest is try/except with return.

def read_config_rusty(path: str) -> Ok_ | Err_:
    """Python: simulating ? operator with manual checks."""
    if not path:
        return Err_("Empty path")

    # Simulating reading a file
    try:
        with open(path) as f:
            content = f.read()
    except FileNotFoundError:
        return Err_("File not found")

    return Ok_(content.strip())


def demo_question_mark():
    """Python: manual early-return pattern."""
    result = read_config_rusty("/nonexistent/file.txt")
    if result.is_ok:
        print(f"  content: {result.value}")
    else:
        print(f"  error: {result.error}")

    # In Rust, with ? operator:
    # fn read_config(path: &str) -> Result<String, std::io::Error> {
    #     let mut file = File::open(path)?;  // ← returns Err if fails
    #     let mut content = String::new();
    #     file.read_to_string(&mut content)?; // ← returns Err if fails
    #     Ok(content)                          // success
    # }

    # The ? operator eliminates all that manual is_ok checking!
    # It's one of Rust's most loved features.


# ============================================================
# SECTION 5: Error propagation — bubbling errors up
# ============================================================
# Python: exceptions bubble up automatically. You catch at the top level.
# Rust:   you must explicitly PROPAGATE errors with ? or match.
#         This is verbose but explicit — you ALWAYS know which functions
#         can fail, because they return Result.

def open_file(path: str) -> Ok_ | Err_:
    """Python: simulating error propagation chain."""
    try:
        with open(path) as f:
            return Ok_(f.read().strip())
    except FileNotFoundError:
        return Err_("open_file: file not found")


def parse_number(content: str) -> Ok_ | Err_:
    """Python: parsing might fail."""
    try:
        return Ok_(int(content))
    except ValueError:
        return Err_("parse_number: not a valid integer")


def read_and_parse(path: str) -> Ok_ | Err_:
    """Python: chaining fallible functions — manual propagation."""
    result = open_file(path)
    if result.is_err:
        return result  # propagate error up

    num_result = parse_number(result.value)
    if num_result.is_err:
        return num_result  # propagate error up

    return Ok_(num_result.value * 2)  # success: double the number


def demo_propagation():
    """Python: manual error propagation chain."""
    result = read_and_parse("/nonexistent/number.txt")
    if result.is_ok:
        print(f"  result: {result.value}")
    else:
        print(f"  error: {result.error}")

    # In Rust, this becomes MUCH cleaner with ?:
    # fn read_and_parse(path: &str) -> Result<i32, String> {
    #     let content = open_file(path)?;       // error? return early
    #     let num = parse_number(&content)?;    // error? return early
    #     Ok(num * 2)
    # }


# ============================================================
# SECTION 6: Custom error types
# ============================================================
# Python: subclass Exception
# Rust:   define an enum implementing std::error::Error or std::fmt::Display
#         Custom error enum wraps all the error types your functions can produce.

class ConfigError(Exception):
    """Python: custom exception for config-related errors."""
    pass


class MissingFileError(ConfigError):
    pass


class ParseError(ConfigError):
    pass


def demo_custom_errors():
    """Python: custom exception hierarchy."""
    try:
        raise MissingFileError("config.toml not found")
    except ConfigError as e:
        print(f"  caught: {e}")

    # Rust equivalent: a custom error enum
    # enum ConfigError {
    #     MissingFile(String),
    #     ParseError(String),
    # }
    # impl std::fmt::Display for ConfigError { ... }
    # impl std::error::Error for ConfigError {}  // optional but recommended

    # Then you can return Result<T, ConfigError> and use ? across
    # functions that return different error types (if they implement From).


# ============================================================
# SECTION 7: .map_err() — converting between error types
# ============================================================
# Rust:   result.map_err(|e| MyError::from(e))?
#         When chaining ? across functions with different error types,
#         you need to convert. The From trait makes this automatic.
#
# Python: type conversion in except blocks, or wrapping.

def demo_map_err():
    """Python: manually converting error types."""
    # Python doesn't have this pattern — exceptions are any type.
    # But you can simulate:
    try:
        raise FileNotFoundError("missing.txt")
    except FileNotFoundError as e:
        wrapped = ConfigError(f"Config load failed: {e}")
        print(f"  converted: {wrapped}")

    # Rust:
    # let content = std::fs::read_to_string("config.toml")
    #     .map_err(|e| ConfigError::MissingFile(e.to_string()))?;
    # // map_err converts std::io::Error into ConfigError so ? can propagate it


# ============================================================
# SECTION 8: main() returning Result
# ============================================================
# Rust:   fn main() -> Result<(), Box<dyn Error>> { ... Ok(()) }
#         This lets you use ? directly in main()!
# Python: No equivalent — you just don't catch exceptions.

def demo_main_result():
    """Python: no equivalent — exceptions just bubble to the top."""
    # In Rust:
    # fn main() -> Result<(), Box<dyn std::error::Error>> {
    #     let content = std::fs::read_to_string("config.toml")?;
    #     //                        ? just works in main ^^^
    #     println!("{content}");
    #     Ok(())
    # }
    # If main returns Err(...), the program prints the error and exits.
    print("  In Python, uncaught exceptions do this automatically.")
    print("  In Rust, you opt in with main() -> Result<...>.")


# ============================================================
# SECTION 9: panic! vs Result — when to use which
# ============================================================
# Python: exceptions for everything
# Rust:
#   panic! → unrecoverable bugs (out-of-bounds, assertion failure, etc.)
#   Result → recoverable errors (file not found, network timeout, bad input)
#
# Rule of thumb: if the CALLER can reasonably handle it → Result.
#                if it means the program is broken → panic!.

def demo_when_to_use():
    """Python: the mental model shift."""

    # Python pattern — exceptions for everything:
    try:
        numbers = [1, 2, 3]
        index = numbers[10]  # IndexError
    except IndexError:
        print("  Python: caught IndexError")

    # Rust pattern:
    # let v = vec![1, 2, 3];
    # v[10];           // PANICS — unrecoverable (programmer bug)
    # v.get(10);       // Returns Option<&i32> — None here, caller handles it

    # Python: parsing user input — exception
    try:
        num = int("not_a_number")
    except ValueError:
        print("  Python: caught ValueError from bad input")

    # Rust: parsing user input — Result
    # let num: Result<i32, _> = "not_a_number".parse();
    # match num {
    #     Ok(n) => println!("{n}"),
    #     Err(e) => println!("bad input: {e}"),
    # }

    print("  Key insight: Python uses try/except reactively.")
    print("  Rust uses Result<T, E> in the TYPE SYSTEM — the compiler")
    print("  forces you to handle errors before the code compiles.")


# ============================================================
# SECTION 10: The Rust mindset — explicit error paths
# ============================================================
# The big takeaway for Python devs:
#
# In Python, a function signature tells you NOTHING about whether it can fail.
#   def read_file(path: str) -> str:
#       ...  # might raise FileNotFoundError, PermissionError, OSError...
#
# In Rust, the return type TELLS YOU it can fail:
#   fn read_file(path: &str) -> Result<String, std::io::Error>
#       ...  // you MUST handle the Err case
#
# This is "making the error path a first-class citizen."
# You can't forget to handle errors — the compiler won't let you.

def demo_rust_mindset():
    """Python: summarizing the mental shift."""
    print("  Python: errors are invisible in type signatures.")
    print("  Rust:   errors are explicit in the return type.")
    print()
    print("  - No more 'what exceptions can this raise?'")
    print("  - No more forgotten try/except blocks")
    print("  - The compiler is your safety net")
    print()
    print("  The ? operator makes error propagation ergonomic.")
    print("  In Python, exceptions bubble automatically.")
    print("  In Rust, you opt INTO propagation with ? — it's explicit.")


if __name__ == "__main__":
    print("=== 1. panic! vs Python exceptions ===")
    demo_panic_equivalent()
    print("\n=== 2. Result<T, E> — errors as return values ===")
    demo_result_pattern()
    print("\n=== 3. unwrap() and expect() ===")
    demo_unwrap_expect()
    print("\n=== 4. The ? operator ===")
    demo_question_mark()
    print("\n=== 5. Error propagation ===")
    demo_propagation()
    print("\n=== 6. Custom error types ===")
    demo_custom_errors()
    print("\n=== 7. map_err() — converting error types ===")
    demo_map_err()
    print("\n=== 8. main() returning Result ===")
    demo_main_result()
    print("\n=== 9. panic! vs Result — when to use which ===")
    demo_when_to_use()
    print("\n=== 10. The Rust mindset — explicit error paths ===")
    demo_rust_mindset()
