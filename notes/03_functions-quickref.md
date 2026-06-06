# Lesson 03 — Functions — Quick Reference

**Completed:** 6th June, 2026 · **Status:** done

Verbose explainer: `03_functions.qmd` (12 sections, render to HTML for full read).

---

## The 5 differences that will trip you up (Python → Rust)

| # | Python | Rust |
|---|---|---|
| 1 | `def` | `fn` |
| 2 | `return` required | last expression (no `;`) is the return |
| 3 | parameter types are hints | **parameter types are mandatory** |
| 4 | `-> int` is a hint | `-> i32` is **enforced** |
| 5 | every line is a statement | `expr` (no `;`) has a value, `expr;` doesn't |

## Naming convention — `snake_case`

```rust
fn calculate_score() {}   // ✅ idiomatic
fn CalculateScore() {}   // ❌ looks like a type
fn printSummary() {}     // ❌ camelCase
```

## The 5 rustlings exercises, one-liner fixes

| Exercise | What to add/fix |
|---|---|
| functions1 | Define a function named `call_me` (anywhere in the file) |
| functions2 | Add a type to the parameter: `num: u8` |
| functions3 | Pass the argument at the call site: `call_me(3)` |
| functions4 | Add the return type: `fn sale_price(price: i64) -> i64` |
| functions5 | Drop the semicolon on the last line: `num * num` not `num * num;` |

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/03_functions.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 03_functions
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/03_functions.rs -o /tmp/03_functions && /tmp/03_functions
```

## The "weeds" — 3 things to internalize

1. **Expressions vs statements** — the #1 mental shift. Read section 2 of the explainer.
2. **if/else is an expression** — no ternary `? :` in Rust; just use `if cond { a } else { b }` and the value is the chosen branch.
3. **No `*args` / `**kwargs`** — use slices: `fn f(items: &[T])` and pass `&[1, 2, 3]`.

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `expected type, found \`)\`` | missing param type | add `: i32` (or whatever) |
| `expected \`i32\`, found \`()`` | trailing `;` on return | drop the `;` |
| `expected \`i64\`, found \`()`` | missing `-> i64` | add return type |
| `this function takes 1 argument but 0 arguments were supplied` | call site missing arg | pass it |
| `cannot find function \`foo\` in this scope` | typo or undefined | define it |

**Read errors top to bottom.** The `help:` line tells you the fix.
