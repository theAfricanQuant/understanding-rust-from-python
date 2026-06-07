"""Lesson 05 — Loops
Python side: while, for, break, continue, for..else
Rust parallels noted in comments."""


def demo_while():
    """while loop — same basic shape as Rust"""
    n = 3
    while n > 0:
        print(f"Python while: {n}")
        n -= 1
    # Python condition can be truthy/falsy too:
    # while n:  (works because 0 is falsy)
    # Rust requires: while n != 0 {}


def demo_while_true():
    """while True — Rust's `loop` is the explicit version of this"""
    n = 0
    while True:
        if n >= 3:
            break
        print(f"Python while True: {n}")
        n += 1


def demo_break_with_value():
    """Python CANNOT break with a value — you need an external variable"""
    n = 0
    result = None  # ← have to declare this OUTSIDE the loop
    while True:
        if n >= 3:
            result = n * 2
            break
        n += 1
    print(f"Python break (need external var): {result}")
    # Rust: let result = loop { if n >= 3 { break n * 2; } n += 1; };
    #       No external variable needed — break carries the value out


def demo_for_range():
    """for with range() — Rust uses 0..n syntax"""
    for i in range(3):
        print(f"Python for range: {i}")
    # Rust: for i in 0..3 { println!("{i}"); }


def demo_for_collection():
    """for over a list — Python always borrows"""
    items = ["a", "b", "c"]
    for item in items:
        print(f"Python for collection: {item}")
    # items is still usable here — Python never consumes
    # Rust: for item in &items {}  (must borrow explicitly with &)
    # Rust: for item in items {}   (consumes — can't use items after)
    print(f"Python — items still usable: {items}")


def demo_enumerate():
    """enumerate — same pattern in Rust with .iter().enumerate()"""
    items = ["a", "b", "c"]
    for i, item in enumerate(items):
        print(f"Python enumerate: {i} -> {item}")
    # Rust: for (i, item) in items.iter().enumerate() { ... }


def demo_continue():
    """continue — same in both languages"""
    for i in range(5):
        if i % 2 == 0:
            continue
        print(f"Python continue (odd only): {i}")


def demo_for_else():
    """for..else — Python-only, no Rust equivalent
    The else block runs only if the loop completed WITHOUT a break."""
    for i in range(3):
        print(f"Python for..else: {i}")
        if i > 5:
            break
    else:
        print("Python for..else: no break occurred (always runs here)")


def demo_loop_labels():
    """Nested breaks — Python has no label syntax
    Workaround: use a flag variable or return from a function."""
    outer_break = False
    for i in range(3):
        for j in range(3):
            if i == 1 and j == 1:
                outer_break = True
                break
            print(f"Python no-labels: ({i}, {j})")
        if outer_break:
            break
    # Rust: 'outer: for i in 0..3 {
    #           for j in 0..3 {
    #               if i == 1 && j == 1 { break 'outer; }
    #               println!("({i}, {j})");
    #           }
    #       }


if __name__ == "__main__":
    print("=== 1. while ===")
    demo_while()
    print("\n=== 2. while True ===")
    demo_while_true()
    print("\n=== 3. break with value ===")
    demo_break_with_value()
    print("\n=== 4. for range ===")
    demo_for_range()
    print("\n=== 5. for collection ===")
    demo_for_collection()
    print("\n=== 6. enumerate ===")
    demo_enumerate()
    print("\n=== 7. continue ===")
    demo_continue()
    print("\n=== 8. for..else ===")
    demo_for_else()
    print("\n=== 9. loop labels (workaround) ===")
    demo_loop_labels()
