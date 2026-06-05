// Lesson 03 — Functions (cargo bin version)
//
// Run with:  cargo run --bin 03_functions
//
// All exercises from rustlings 02_functions adapted.

fn main() {
    println!("== functions1: define a function ==");
    call_me();

    println!("\n== functions2: type the parameter ==");
    call_me_typed(3);

    println!("\n== functions3: pass the argument ==");
    call_me_with_arg(3);

    println!("\n== functions4: type the return ==");
    let price = sale_price(51);
    println!("Your sale price is {price}");

    println!("\n== functions5: drop the semicolon ==");
    let sq = square(3);
    println!("The square of 3 is {sq}");
}

// --- functions1: no args, no return ---
fn call_me() {
    println!("Ring! Call number 1");
    println!("Ring! Call number 2");
}

// --- functions2: parameter needs a type ---
fn call_me_typed(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// --- functions3: pass the argument at the call site ---
fn call_me_with_arg(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// --- functions4: return type goes after `->` ---
fn is_even(num: i64) -> bool {
    num % 2 == 0
}

fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10      // expression, no `;`
    } else {
        price - 3       // expression, no `;`
    }
    // The if/else block itself is an expression whose value is the chosen branch
}

// --- functions5: implicit return — last expression (no `;`) is the return value ---
fn square(num: i32) -> i32 {
    num * num   // ← no semicolon! this is the return value
    // If you wrote `num * num;` it would return ()
}
