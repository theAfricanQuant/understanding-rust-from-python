"""Lesson 06 — Ownership & Borrowing
Python side: demonstrating what Python does INSTEAD of ownership.
Rust's ownership has NO equivalent in Python — this file shows the contrast."""

import sys


def demo_python_no_ownership():
    """Python: multiple names can point to the same object.
    Rust: let s2 = s1; moves the value — s1 is DEAD."""
    s1 = "hello"
    s2 = s1  # Python: s2 now points to same string as s1
    print(f"Python — both alive after 'assign': s1={s1}, s2={s2}")
    # In Rust: let s2 = s1; println!("{s1}");  // ERROR: use of moved value


def demo_refcount():
    """Python tracks references at runtime.
    Rust tracks ownership at compile time — zero runtime cost."""
    a = [1, 2, 3]
    print(f"  refcount after a = [1,2,3]: {sys.getrefcount(a) - 1}")

    b = a
    print(f"  refcount after b = a: {sys.getrefcount(a) - 1}")

    c = b
    print(f"  refcount after c = b: {sys.getrefcount(a) - 1}")

    del b
    print(f"  refcount after del b: {sys.getrefcount(a) - 1}")
    # Rust: compile-time tracking, no refcount, no GC, no runtime cost


def demo_mutable_shared():
    """Python: shared references to mutable objects can cause surprises.
    Rust: prevents this at compile time."""
    inner = [1, 2, 3]
    outer = [inner, inner, inner]  # all three point to the SAME list
    inner.append(4)  # modifies all three at once
    print(f"Python shared mutation: {outer}")
    # Rust: cannot have multiple &mut references to the same Vec

    # In Rust, you'd need to clone explicitly:
    # let inner = vec![1, 2, 3];
    # let outer = vec![inner.clone(), inner.clone(), inner.clone()];


def demo_scope():
    """Python scoping — variables exist until function returns or del.
    Rust: values are DROPPED when the variable goes out of scope (RAII)."""
    x = "I exist"
    print(f"  before block: {x}")
    if True:
        y = "I'm inside the block"
        print(f"    inside block: {y}")
    # print(y)  # would error — y doesn't exist here
    print(f"  after block: {x}")
    # Rust: the same, but Rust also calls drop() when x goes out of scope
    # freeing resources immediately. No GC, no waiting, no cleanup() call.


def demo_python_clone():
    """Python: sometimes you explicitly copy to avoid shared mutation.
    Rust: MOVES by default, you .clone() if you want a copy."""
    original = [1, 2, 3]
    # Python: assignment is NOT a copy
    alias = original
    alias.append(4)
    print(f"  original after alias.append: {original}")  # mutated!

    # Python: explicit copy
    original2 = [1, 2, 3]
    copy = original2.copy()
    copy.append(4)
    print(f"  original after copy.append: {original2}")  # unchanged
    # Rust default is MOVE not share, so the 'alias problem' doesn't exist
    # Rust: let original = vec![1,2,3];
    #       let moved = original;         // original is dead
    #       // let alias = &original;     // ERROR: moved


def demo_tuple_vs_list():
    """Python: tuples are immutable, lists are mutable.
    Rust: mutability is a property of the BINDING, not the type."""
    # Python: tuple can't be changed
    t = (1, 2, 3)
    # t[0] = 99  # TypeError

    # Python: list can be changed
    lst = [1, 2, 3]
    lst[0] = 99

    # Rust: let t = (1, 2, 3);   → immutable binding
    #       let mut t = (1, 2, 3); → mutable binding
    # The tuple itself is the same type — the binding decides if it's mutable


if __name__ == "__main__":
    print("=== 1. No ownership in Python ===")
    demo_python_no_ownership()
    print("\n=== 2. Reference counting ===")
    demo_refcount()
    print("\n=== 3. Shared mutation ===")
    demo_mutable_shared()
    print("\n=== 4. Scoping ===")
    demo_scope()
    print("\n=== 5. Copy vs assign ===")
    demo_python_clone()
    print("\n=== 6. Mutability ===")
    demo_tuple_vs_list()
