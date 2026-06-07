/// Lesson 06 — Ownership & Borrowing
/// Rust side: moves, clones, borrows, slices
///
/// This is THE Rust lesson. If you understand this, everything else clicks.
/// Python does nothing like this — ownership is 100% new.
///
/// The 3 rules of ownership:
///   1. Each value has exactly ONE owner
///   2. When the owner goes out of scope, the value is DROPPED
///   3. You can either: one &mut (mutable borrow) OR many & (immutable borrows)

/// ---- SECTION 1: Move semantics ----
/// In Python: let y = x;  → both y and x point to same object
/// In Rust:   let y = x;  → the value MOVES from x to y, x is dead

#[derive(Debug, Clone)]
struct House {
    name: String,
}

fn demo_move() {
    let h1 = House {
        name: String::from("Blue House"),
    };
    let h2 = h1; // ← MOVES: ownership transfers to h2

    // println!("{h1:?}");  // ❌ ERROR: use of moved value
    println!("move: owner is now h2 -> {h2:?}");

    // Python: h1 and h2 both point to the same house. Both alive.
    // Rust:   h1 is DEAD after move. Only h2 owns it.
    //         The compiler tracks this — no runtime cost.
}

/// ---- SECTION 2: Clone ----
/// If you want BOTH to be usable, you must .clone() explicitly.

fn demo_clone() {
    let h1 = House {
        name: String::from("Red House"),
    };
    let h2 = h1.clone(); // ← DEEP COPY

    println!("clone: h1 still alive -> {h1:?}");
    println!("clone: h2 is a copy -> {h2:?}");
    // Both alive. Two independent houses.
}

/// ---- SECTION 3: Copy types ----
/// Simple stack-only types (integers, bool, char, floats) implement Copy.
/// They don't move — they copy implicitly.

fn demo_copy() {
    let x: i32 = 42;
    let y: i32 = x; // ← COPY, not move (i32 implements Copy)

    println!("copy: x = {x}, y = {y}"); // both alive
    // Integers, bool, char, floats, tuples of Copy types → Copy
    // String, Vec, structs with non-Copy fields → MOVE by default
}

/// ---- SECTION 4: Borrowing (&) ----
/// Borrow = temporary access without ownership.
/// You give someone the key to look, but you still own the house.

fn look_at(house: &House) {
    // house is a REFERENCE (&House). We can READ but not move/destroy.
    println!("  visitor sees: {house:?}");
    // house is dropped here — but it's just a reference, the real value lives on
}

fn demo_borrow() {
    let house = House {
        name: String::from("Green House"),
    };

    look_at(&house); // ← borrow: house is NOT moved, we gave a reference
    println!("borrow: owner still has it -> {house:?}"); // ✅ still alive

    // look_at(house);  // ← would MOVE: house would be dead after
}

/// ---- SECTION 5: Mutable borrowing (&mut) ----
/// You give someone a renovation key — they can change things.
/// But only ONE renovation key can exist at a time.

fn renovate(house: &mut House) {
    house.name.push_str(" (renovated)");
}

fn demo_mut_borrow() {
    let mut house = House {
        name: String::from("White House"),
    };

    renovate(&mut house);
    println!("mut borrow: {house:?}"); // "White House (renovated)"

    // ---- The critical rule ----
    // You can have EITHER:
    //   - one &mut (mutable borrow)
    //   - OR many & (immutable borrows)
    // But NOT both at the same time.

    let ref1: &House = &house;
    let ref2: &House = &house;
    println!("  many &: {ref1:?} and {ref2:?}"); // ✅ many & is OK

    // let mut_ref: &mut House = &mut house;  // ❌ ERROR: can't borrow as mutable
    // println!("  read after mut? {ref1}");   //    because &ref1 and &ref2 are alive
}

/// ---- SECTION 6: The borrow checker in action ----

fn demo_borrow_checker() {
    let mut s = String::from("hello");

    // Rule: many & is fine
    let r1 = &s;
    let r2 = &s;
    println!("  many &: r1={r1}, r2={r2}");

    // The & references are no longer needed after last use above.
    // Now we can borrow mut:

    let r3 = &mut s;
    r3.push_str(" world");
    println!("  one &mut: {r3}");

    // What you CAN'T do:
    // let a = &s;        // immutable borrow
    // let b = &mut s;    // ❌ can't mix & and &mut
    // println!("{a} {b}");
}

/// ---- SECTION 7: Slices ----
/// A slice is a VIEW into a collection — it borrows without owning.

fn demo_slices() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &arr[1..4]; // view of [20, 30, 40]

    println!("slices: arr={arr:?}");
    println!("slices: slice=&arr[1..4] => {slice:?}");
    // slice borrows arr — arr is still the owner, still alive
}

/// ---- SECTION 8: Dangling reference prevention ----
/// Rust prevents dangling references at compile time.

// fn demo_dangle() -> &String {  // ❌ ERROR: missing lifetime specifier
//     let s = String::from("hello");
//     &s  // ← s is dropped when function ends, reference would dangle
// }
// The compiler catches this. In Python, you'd get None or an error at runtime.

fn demo_no_dangle() -> String {
    let s = String::from("hello");
    s // ← MOVES ownership to the caller, no dangling
}

fn main() {
    println!("=== 1. Move semantics ===");
    demo_move();
    println!("\n=== 2. Clone ===");
    demo_clone();
    println!("\n=== 3. Copy types ===");
    demo_copy();
    println!("\n=== 4. Borrowing (&) ===");
    demo_borrow();
    println!("\n=== 5. Mutable borrowing (&mut) ===");
    demo_mut_borrow();
    println!("\n=== 6. Borrow checker ===");
    demo_borrow_checker();
    println!("\n=== 7. Slices ===");
    demo_slices();
    println!("\n=== 8. No dangling references ===");
    let s = demo_no_dangle();
    println!("  no dangle: {s}");
}
