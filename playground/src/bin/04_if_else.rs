// Lesson 04 — If/else (cargo bin version)
//
// Run with:  cargo run --bin 04_if_else
//
// The 3 rustlings 03_if exercises adapted.

fn main() {
    println!("== if1: bigger ==");
    println!("bigger(10, 8)   = {}", bigger(10, 8));
    println!("bigger(32, 42)  = {}", bigger(32, 42));
    println!("bigger(42, 42)  = {}", bigger(42, 42));

    println!("\n== if2: picky eater (all branches same type) ==");
    println!("picky_eater(\"strawberry\") = {}", picky_eater("strawberry"));
    println!("picky_eater(\"potato\")     = {}", picky_eater("potato"));
    println!("picky_eater(\"broccoli\")   = {}", picky_eater("broccoli"));
    println!("picky_eater(\"gummy bears\") = {}", picky_eater("gummy bears"));

    println!("\n== if3: animal habitat (let-binding an if expression) ==");
    println!("crab     -> {}", animal_habitat("crab"));
    println!("gopher   -> {}", animal_habitat("gopher"));
    println!("snake    -> {}", animal_habitat("snake"));
    println!("dinosaur -> {}", animal_habitat("dinosaur"));
}

// --- if1: return the bigger number using if/else as an expression ---
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// --- if2: ALL BRANCHES must return the same type ---
// The broken version had `"Yummy!"` in the if-branch and `1` in the else-branch.
// Strings and ints are different types — Rust rejects the whole block.
//
// Fix: every branch returns `&'static str`.
fn picky_eater(food: &str) -> &'static str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

// --- if3: an if/else block CAN be assigned to a variable with `;` ---
// Rust requires every branch of the if/else to have the same type
// (the if/else is a single expression with one type).
//
// The broken version mixed `1`, `2.0`, `3`, and `"Unknown"` — different types.
// Fix: pick ONE type for all branches. Here we use `&'static str`.
fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        "1"
    } else if animal == "gopher" {
        "2"
    } else if animal == "snake" {
        "3"
    } else {
        "0"
    };

    // Now the second if/else maps the string ID to a habitat
    if identifier == "1" {
        "Beach"
    } else if identifier == "2" {
        "Burrow"
    } else if identifier == "3" {
        "Desert"
    } else {
        "Unknown"
    }
}
