/// Lesson 05 — Loops
/// Rust side: loop, while, for, break with value, continue, loop labels
/// Compile: rustc rust/05_loops.rs -o /tmp/05_loops && /tmp/05_loops

fn demo_loop() {
    // `loop` is Rust-only — explicit infinite block, no condition
    // Python would write: while True:
    let mut n: u32 = 0;
    loop {
        if n >= 3 {
            break;
        }
        println!("loop: {n}");
        n += 1;
    }
}

fn demo_break_with_value() {
    // Rust-only trick: break RETURNS a value from the loop
    // Python needs an external variable (see python/05_loops.py)
    let mut n: u32 = 0;
    let result: u32 = loop {
        if n >= 3 {
            break n * 2;  // ← this IS the loop's return value
        }
        n += 1;
    };
    println!("break with value: {result}");
    // No external variable needed — break carries it out
}

fn demo_while() {
    // while — same basic shape as Python
    // Rule: condition must be bool (same rule as if from lesson 04)
    let mut n: i32 = 3;
    while n > 0 {
        println!("while: {n}");
        n -= 1;
    }
}

fn demo_for_range() {
    // for over range — Python would write: for i in range(3):
    // Rust uses the .. operator (exclusive upper bound)
    for i in 0..3 {
        println!("for range: {i}");
    }
    // 0..=3 would include 3 (inclusive range)
}

fn demo_for_collection() {
    // for over a collection — MUST decide: borrow or consume
    let items: [&str; 3] = ["a", "b", "c"];

    println!("  -- borrowing (items still usable) --");
    for item in &items {
        // &items gives us & &str references — we borrow, don't take ownership
        println!("  {item}");
    }
    println!("  items still alive: {:?}", items);

    // If we used `for item in items {}` instead of &items,
    // items would move into the loop and we couldn't use it after.
    // Try it: change &items to items and see the compiler error.
}

fn demo_enumerate() {
    // enumerate — same pattern, slightly different syntax
    let items: [&str; 3] = ["a", "b", "c"];
    for (i, item) in items.iter().enumerate() {
        // Python: for i, item in enumerate(items):
        println!("  {i} -> {item}");
    }
}

fn demo_continue() {
    // continue — same as Python, skips to next iteration
    for i in 0..5 {
        if i % 2 == 0 {
            continue;
        }
        println!("continue (odd only): {i}");
    }
}

fn demo_loop_labels() {
    let mut count: u32 = 0;
    'outer: loop {
        loop {
            if count >= 2 {
                break 'outer;
            }
            count += 1;
        }
    }
    println!("loop labels: count = {count} (broke from 'outer)");
}

fn demo_no_for_else() {
    // Python's for..else has NO equivalent in Rust
    // Python:
    //   for i in range(3):
    //       if i > 5: break
    //   else:
    //       print("no break")
    //
    // Rust: use a flag variable instead
    let mut broke: bool = false;
    for i in 0..3 {
        if i > 5 {
            broke = true;
            break;
        }
    }
    if !broke {
        println!("no for..else: no break occurred (Rust: use a flag)");
    }
}

fn main() {
    println!("=== 1. loop ===");
    demo_loop();
    println!("\n=== 2. break with value ===");
    demo_break_with_value();
    println!("\n=== 3. while ===");
    demo_while();
    println!("\n=== 4. for range ===");
    demo_for_range();
    println!("\n=== 5. for collection ===");
    demo_for_collection();
    println!("\n=== 6. enumerate ===");
    demo_enumerate();
    println!("\n=== 7. continue ===");
    demo_continue();
    println!("\n=== 8. loop labels ===");
    demo_loop_labels();
    println!("\n=== 9. no for..else ===");
    demo_no_for_else();
}
