# Rust from Python 🐍 → 🦀

A learning journal: **side-by-side Python and Rust**, built from a Pythonista's perspective.

> **First time cloning this repo?** Read [`SETUP.md`](./SETUP.md) for the full step-by-step.

## Structure

| Folder | What's in it |
|---|---|
| `learn-rust.qmd` | The side-by-side lesson document (render with Quarto) |
| `python/` | Python reference scripts (one per lesson) |
| `rust/` | Plain `.rs` files (compile with `rustc`) |
| `playground/` | Cargo multi-bin project (`cargo run --bin XX_name`) |
| `notes/` | Quick markdown notes per topic |

## Quick start (TL;DR — full guide in [SETUP.md](./SETUP.md))

```bash
# Python
uv sync
uv run python python/01_hello.py

# Rust via cargo
cd playground && cargo run --bin 01_hello && cd ..

# Rust standalone
rustc rust/01_hello.rs -o /tmp/h && /tmp/h

# Render the doc
quarto render learn-rust.qmd --to html
```

## Lessons done so far

| # | Topic | Date | Status |
|---|---|---|---|
| 01 | Hello world | 2026-06-05 | ✅ done |
| 02 | Variables (`let`, `mut`, types, shadowing, `const`) | 2026-06-05 | ✅ done |
| 03 | Functions | — | ⬜ pending |
| 04 | Control flow | — | ⬜ pending |
| 05 | Loops | — | ⬜ pending |
| 06 | Ownership & borrowing | — | ⬜ pending |
| 07 | Structs & enums | — | ⬜ pending |
| 08 | Error handling | — | ⬜ pending |

## Why this exists

I'm a Python developer learning Rust. I keep forgetting Rust rules because Python lets me get away with everything. This repo is my cheat sheet — one lesson at a time, Python first, Rust second, side by side.
