# Lesson 02 — Variables

**Completed:** 5th June, 2026 · **Status:** done

Quick reference. Full lesson: `../../learn-rust.qmd` (lesson 02).

## The 5 rules

| Rule | Python | Rust |
|---|---|---|
| Declare | `x = 5` | `let x = 5;` |
| Mutate | always | `let mut x = 5;` (opt-in) |
| Type | dynamic | static, inferred or annotated |
| Reassign | rebind | shadow with `let` |
| Constant | convention `PI = 3.14` | `const PI: f64 = 3.14;` |

## Run it

```bash
cd playground && cargo run --bin 02_vars
python3 python/02_vars.py
```
