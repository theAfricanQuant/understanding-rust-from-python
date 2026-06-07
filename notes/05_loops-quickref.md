# Lesson 05 — Loops — Quick Reference

**Completed:** 7th June, 2026 · **Status:** done

Verbose explainer: `05_loops.qmd` (12 sections, stories, into the weeds).

---

## The 3 loop types in Rust vs Python

| Loop type | Python | Rust |
|---|---|---|
| Infinite | `while True:` | `loop { }` |
| Condition-based | `while n > 0:` | `while n > 0 { }` |
| Range iteration | `for i in range(3):` | `for i in 0..3 { }` |
| Collection iteration | `for x in items:` | `for x in &items { }` |

---

## The 3 Rust-only features

| Feature | Example | Python replacement |
|---|---|---|
| `break` with a value | `let x = loop { break 42; };` | external variable before loop |
| Loop labels | `'outer: for ... { break 'outer; }` | flag variable or function return |
| `for` ownership choice | `for x in items {}` vs `for x in &items {}` | Python always borrows |

---

## The big insight: `break` carries a value

**Rust:**
```rust
let result = loop {
    if done { break value; }
};
```

**Python:**
```python
result = None
while True:
    if done:
        result = value
        break
```

Rust eliminates the external scaffolding variable.

---

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/05_loops.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 05_loops
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/05_loops.rs -o /tmp/05_loops && /tmp/05_loops
```

---

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `expected \`bool\`, found integer` | `while` condition not bool | use a comparison: `while n != 0` |
| `use of moved value: \`items\`` | consumed collection in `for` | use `&items` to borrow instead |
| `can't break with value` | `break val` used in `while` or `for` | only `loop` supports break with value |
| `unused label` | declared label but never used | remove the `'label:` or use it with `break` |

---

## Key rules

- `break with value` only works inside `loop { }`, NOT in `while` or `for`
- `loop` is the *only* loop type in Rust that returns a value
- `for x in items {}` *consumes* items (moves ownership)
- `for x in &items {}` *borrows* — items still usable after
- `while` condition must be `bool` (same rule as `if`)
- Python's `for..else` has NO Rust equivalent — use a flag variable
- `0..3` is exclusive (0, 1, 2); `0..=3` is inclusive (0, 1, 2, 3)
