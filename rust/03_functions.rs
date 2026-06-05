// Lesson 03 — Functions
// Standalone .rs (compile with: rustc rust/03_functions.rs -o /tmp/h && /tmp/h)
//
// Key Rust rules in this lesson:
//   - fn name(params) -> return_type { body }
//   - Parameter types REQUIRED (no inference for params)
//   - No default arguments in plain fn (use trait/impl tricks later)
//   - No *args/**kwargs (use slices or macros)
//   - Last expression (no semicolon) is the return value
//   - Multiple returns: use a tuple

fn main() {
    // 1. Basic function call
    greet();

    // 2. Function with parameters
    greet_person("Siseng", "Hello");

    // 3. Function with a return value
    let result = add(3, 4);
    println!("3 + 4 = {result}");

    // 4. Multiple return values: tuple
    let (lo, hi) = min_and_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={lo}, max={hi}");

    // 5. No *args/**kwargs — use slices instead
    print_all(&["a", "b", "c"]);

    // 6. Functions as values (function pointers)
    let answer = apply(square, 7);
    println!("apply(square, 7) = {answer}");
}

// 1. Basic function: no args, no return (unit type `()`)
fn greet() {
    println!("Hello!");
}

// 2. Function with parameters — types REQUIRED
fn greet_person(name: &str, greeting: &str) {
    println!("{greeting}, {name}!");
}

// 3. Function with a return value — last expression (no `;`) is the return
fn add(a: i32, b: i32) -> i32 {
    a + b         // ← no semicolon! this is the return value
    // If you put `;` here, it becomes a statement and the function returns ()
}

// 4. Multiple return values: return a tuple
fn min_and_max(numbers: &[i32]) -> (i32, i32) {
    let mut lo = numbers[0];
    let mut hi = numbers[0];
    for &n in numbers {
        if n < lo { lo = n; }
        if n > hi { hi = n; }
    }
    (lo, hi)       // tuple — returned as the last expression
}

// 5. Variable number of args — Rust uses slices, not varargs
fn print_all(items: &[&str]) {
    for item in items {
        print!("{item} ");
    }
    println!();
}

// 6. Type-annotated function (mandatory in Rust, unlike Python)
fn square(num: i32) -> i32 {
    num * num
}

// 7. Functions are values — pass them as arguments
//    `fn(i32) -> i32` is the type of a function pointer
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
