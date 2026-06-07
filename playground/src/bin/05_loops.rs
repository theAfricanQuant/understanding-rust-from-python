/// Lesson 05 — Loops (cargo version)
/// Run: cargo run --bin 05_loops

fn demo_loop() {
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
    let mut n: u32 = 0;
    let result: u32 = loop {
        if n >= 3 {
            break n * 2;
        }
        n += 1;
    };
    println!("break with value: {result}");
}

fn demo_while() {
    let mut n: i32 = 3;
    while n > 0 {
        println!("while: {n}");
        n -= 1;
    }
}

fn demo_for_range() {
    for i in 0..3 {
        println!("for range: {i}");
    }
}

fn demo_for_collection() {
    let items: [&str; 3] = ["a", "b", "c"];
    for item in &items {
        println!("for borrow: {item}");
    }
    println!("items still alive: {:?}", items);
}

fn demo_enumerate() {
    let items: [&str; 3] = ["a", "b", "c"];
    for (i, item) in items.iter().enumerate() {
        println!("  {i} -> {item}");
    }
}

fn demo_continue() {
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
    let mut broke = false;
    for i in 0..3 {
        if i > 5 {
            broke = true;
            break;
        }
    }
    if !broke {
        println!("no for..else in Rust — use a flag");
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
