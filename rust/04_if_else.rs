// Lesson 04 — If/else (standalone, compile with rustc)
//
// Key Rust rules:
//   - if is an EXPRESSION, not a statement (like Python 3 ternaries)
//   - Condition must be a bool — no truthy/falsy like Python
//   - All branches of if/else must have the SAME TYPE
//   - No `elif` — use `else if`
//   - No ternary `? :` — use the if expression directly

fn main() {
    // 1. Basic if/else as an expression (returns a value)
    let winner = bigger(10, 8);
    println!("bigger(10, 8) = {winner}");

    // 2. Conditions must be bool — 0, "", [], None are NOT falsy
    //    Uncomment the next two lines to see the compile error:
    //
    //    let n = 0;
    //    if n { println!("zero is truthy in Rust"); }   // ❌ expected `bool`
    let n = 0;
    let n_is_zero = n == 0;       // explicit comparison → bool
    if n_is_zero {
        println!("n is zero (but only because we compared it)");
    }

    // 3. if/else if/else chains (no `elif` keyword in Rust)
    let grade = grade(82);
    println!("grade(82) = {grade}");

    // 4. if as a value (no ternary operator)
    let s = sign(5);
    println!("sign(5) = {s}");
    let s = sign(-3);
    println!("sign(-3) = {s}");

    // 5. Nested if
    for &n in &[5, -3, 0] {
        let d = describe_number(n);
        println!("describe_number({n}) = {d}");
    }

    // 6. The whole if/else block can be assigned to a let
    let animal = "crab";
    let habitat = if animal == "crab" {
        "Beach"
    } else if animal == "gopher" {
        "Burrow"
    } else if animal == "snake" {
        "Desert"
    } else {
        "Unknown"
    };
    println!("{animal} lives in {habitat}");
}

// 1. if/else as an expression — return value is the chosen branch
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }       // ← the WHOLE block is the return value
}

// 3. else-if chains (no `elif`)
fn grade(score: i32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}

// 4. "ternary" — just an if expression on one line
fn sign(n: i32) -> &'static str {
    if n > 0 { "positive" } else { "non-positive" }
}

// 5. Nested if
fn describe_number(n: i32) -> &'static str {
    if n != 0 {
        if n > 0 { "positive non-zero" } else { "negative non-zero" }
    } else {
        "zero"
    }
}
