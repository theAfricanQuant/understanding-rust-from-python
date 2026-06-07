# Lesson 06 — Ownership & Borrowing — Quick Reference

**Completed:** 7th June, 2026 · **Status:** done

Verbose explainer: `06_ownership.qmd` (12 sections, 3 stories, into the weeds).

---

## The 3 rules of ownership

| # | Rule |
|---|---|
| 1 | Each value has exactly **one owner** |
| 2 | When the owner goes out of scope, the value is **dropped** |
| 3 | Either: one **&mut** (mutable borrow) OR many **&** (immutable borrows), but not both at the same time |

---

## The type-based behavior

| Type | `let y = x;` does | Can use x after? |
|---|---|---|
| Integer (`i32`, `u32`, etc.) | **Copy** — x is copied to y | ✅ yes |
| `bool`, `char`, `f32`, `f64` | **Copy** — same | ✅ yes |
| `String`, `Vec<T>`, most structs | **Move** — value moves to y | ❌ no — use of moved value |
| Any Copy tuple: `(i32, bool)` | **Copy** — all fields are Copy | ✅ yes |

**To keep both usable on a non-Copy type:** `let y = x.clone();`

---

## The 3 reference types

| Reference | Read? | Write? | How many allowed? |
|---|---|---|---|
| `&T` | ✅ yes | ❌ no | **many** |
| `&mut T` | ✅ yes | ✅ yes | **one** |

---

## The borrow checker rule (memorize this)

> **You cannot borrow mutably while any immutable references are still alive.**

```rust
let r1 = &x;       // ✅ immutable
let r2 = &x;       // ✅ second immutable — fine
// let r3 = &mut x; // ❌ error: already borrowed as immutable
println!("{r1}");  // r1's borrow is active until last use
// → after last use of r1/r2, &mut is fine
```

---

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/06_ownership.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 06_ownership
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/06_ownership.rs -o /tmp/06_ownership && /tmp/06_ownership
```

---

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `use of moved value: \`x\`` | used variable after move | use `.clone()` or reorganize code |
| `cannot borrow \`x\` as immutable/mutable` | borrow rule violation | check active references, release before new borrow |
| `cannot move out of borrowed content` | tried to move from `&` | you only have a reference, copy/clone instead |
| `expected \`&mut T\` but found \`&\` T` | gave immutable ref to mut function | change function sig or use `&mut` |
| `returns a reference to data owned by function` | dangling reference | return owned value, not reference |

---

## Key rules

- **Move is the default** for heap types (`String`, `Vec`, custom structs)
- **Copy is opt-in** via the `Copy` trait (integers, bool, char — anything stack-only)
- **Borrowing (`&`) doesn't transfer ownership** — it's a temporary loan
- **References are non-owning pointers** — the borrower doesn't have to drop the value
- **Slices (`&[T]`, `&str`)** are just borrowed views of contiguous data
- **Rust has no GC, no refcount** — ownership is tracked at compile time with zero runtime cost
