# Lesson 04 — If/else — Quick Reference

**Completed:** 6th June, 2026 · **Status:** done

Verbose explainer: `04_if_else.qmd` (13 sections, render to HTML for full read).

---

## The 3 differences that will trip you up (Python → Rust)

| # | Python | Rust |
|---|---|---|
| 1 | `if` is a **statement** — no value | `if` is an **expression** — every branch returns a value |
| 2 | Condition can be truthy/falsy | Condition **must be `bool`** |
| 3 | `elif` | `else if` (two words) |

## The truthiness translation table

| Python | Rust |
|---|---|
| `if n:` | `if n != 0:` |
| `if s:` | `if !s.is_empty():` |
| `if lst:` | `if !lst.is_empty():` |
| `if x is None:` | `if x.is_none():` (for `Option<T>`) |

## The 3 rustlings exercises, one-liner fixes

| Exercise | What to add/fix |
|---|---|
| if1 | Use if/else as expression: `if a > b { a } else { b }` |
| if2 | All branches must return the same type (use `&'static str` for all) |
| if3 | All 4 branches of the if/else must be the same type (use `i32` for all) |

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/04_if_else.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 04_if_else
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/04_if_else.rs -o /tmp/04_if_else && /tmp/04_if_else
```

## The "weeds" — 3 things to internalize

1. **`if` returns a value** — the #1 mental shift. This replaces the ternary `? :` operator. Use `let x = if cond { a } else { b };`.
2. **Conditions must be `bool`** — no `if 0`, `if ""`, `if []`. Always be explicit: `if !list.is_empty()`.
3. **All branches must agree on a type** — if the if-branch is `i32`, every else-branch must also be `i32`. If they don't agree, the compiler rejects the whole block.

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `expected \`bool\`, found \`{integer}\`` | non-bool condition | use a comparison (`n != 0`) |
| `\`if\` and \`else\` have incompatible types` | branches differ in type | pick one type, make all branches match |
| `expected \`i32\`, found \`()'\`` | missing value in a branch | add an expression, or use `else` |
| `if may be missing an else clause` | used if as value without else | add an `else` branch |

**Read errors top to bottom.** The `help:` line tells you the fix.
