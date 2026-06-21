# Rust from Python 🐍 → 🦀

A learning journal: **side-by-side Python and Rust**, built from a Pythonista's perspective.

> **First time cloning this repo?** Read [`SETUP.md`](./SETUP.md) for the full step-by-step.

## Structure

| Folder | What's in it |
|---|---|
| `notes/` | Rendered `.qmd` lesson explainers (one per lesson, render to standalone HTML) |
| `python/` | Python reference scripts (one per lesson) |
| `rust/` | Plain `.rs` files (compile with `rustc`) |
| `playground/` | Cargo multi-bin project (`cargo run --bin XX_name`) |
| `notes/` | Quick markdown notes per topic |
| `SETUP.md` | First-time-clone instructions |

## Quick start (TL;DR — full guide in [SETUP.md](./SETUP.md))

```bash
# Python
uv sync
uv run python python/01_hello.py

# Rust via cargo
cd playground && cargo run --bin 01_hello && cd ..

# Rust standalone
rustc rust/01_hello.rs -o /tmp/h && /tmp/h

# Render a lesson
uv run quarto render notes/XX_topic.qmd --to html
```

## Lessons done so far

| # | Topic | Date | Status |
|---|---|---|---|
| 01 | Hello world | 5th June, 2026 | ✅ done |
| 02 | Variables (`let`, `mut`, types, shadowing, `const`) | 5th June, 2026 | ✅ done |
| 03 | Functions | 6th June, 2026 | ✅ done |
| 04 | If/else (expression, bool-only, same-type branches) | 6th June, 2026 | ✅ done |
| 05 | Loops (loop, while, for, break with value, labels) | 7th June, 2026 | ✅ done |
| 06 | Ownership & borrowing (move, clone, copy, &, &mut, slices) | 7th June, 2026 | ✅ done |
| 07 | Structs & enums (struct, impl, enum, match, Option, if let) | 14th June, 2026 | ✅ done |
| 08 | Error handling | 21st June, 2026 | ✅ done |

## Why this exists

I'm a Python developer learning Rust. I keep forgetting Rust rules because Python lets me get away with everything. This repo is my cheat sheet — one lesson at a time, Python first, Rust second, side by side.
