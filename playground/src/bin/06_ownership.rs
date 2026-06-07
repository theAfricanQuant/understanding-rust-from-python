/// Lesson 06 — Ownership & Borrowing (cargo version)
/// Run: cargo run --bin 06_ownership

#[derive(Debug, Clone)]
struct House {
    name: String,
}

fn demo_move() {
    let h1 = House {
        name: String::from("Blue House"),
    };
    let h2 = h1;
    println!("move: owner is now h2 -> {h2:?}");
}

fn demo_clone() {
    let h1 = House {
        name: String::from("Red House"),
    };
    let h2 = h1.clone();
    println!("clone: h1 -> {h1:?}, h2 -> {h2:?}");
}

fn demo_copy() {
    let x: i32 = 42;
    let y: i32 = x;
    println!("copy: x={x}, y={y}");
}

fn look_at(house: &House) {
    println!("  visitor sees: {house:?}");
}

fn demo_borrow() {
    let house = House {
        name: String::from("Green House"),
    };
    look_at(&house);
    println!("  owner still has it: {house:?}");
}

fn renovate(house: &mut House) {
    house.name.push_str(" (renovated)");
}

fn demo_mut_borrow() {
    let mut house = House {
        name: String::from("White House"),
    };
    renovate(&mut house);
    println!("  after renovate: {house:?}");

    let r1 = &house;
    let r2 = &house;
    println!("  many &: {r1:?} and {r2:?}");
}

fn demo_borrow_checker() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("  many &: r1={r1}, r2={r2}");

    let r3 = &mut s;
    r3.push_str(" world");
    println!("  one &mut: {r3}");
}

fn demo_slices() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4];
    println!("  arr={arr:?}, slice={slice:?}");
}

fn demo_no_dangle() -> String {
    String::from("hello")
}

fn main() {
    println!("=== 1. Move ===");
    demo_move();
    println!("\n=== 2. Clone ===");
    demo_clone();
    println!("\n=== 3. Copy ===");
    demo_copy();
    println!("\n=== 4. Borrow ===");
    demo_borrow();
    println!("\n=== 5. Mut borrow ===");
    demo_mut_borrow();
    println!("\n=== 6. Borrow checker ===");
    demo_borrow_checker();
    println!("\n=== 7. Slices ===");
    demo_slices();
    println!("\n=== 8. No dangling ===");
    let s = demo_no_dangle();
    println!("  {s}");
}
