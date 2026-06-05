# Rust from Python 🐍 → 🦀

A learning journal: **side-by-side Python and Rust**, built from a Pythonista's perspective.

## Structure

| Folder | What's in it |
|---|---|
| `learn-rust.qmd` | The side-by-side lesson document (render with Quarto) |
| `python/` | Python reference scripts (one per lesson) |
| `rust/` | Plain `.rs` files (compile with `rustc`) |
| `playground/` | Cargo multi-bin project (`cargo run --bin XX_name`) |
| `notes/` | Quick markdown notes per topic |

## Quick start

### Python side (uv-managed)

```bash
cd rust-from-python
uv sync                  # creates .venv and installs everything
uv run jupyter lab       # or whatever you want
```

### Rust side (cargo)

```bash
cd playground
cargo run --bin 01_hello
cargo run --bin 02_vars
```

### Render the lesson document

```bash
quarto render learn-rust.qmd --to html
```

## Lessons done so far

- [x] **01** — Hello world
- [x] **02** — Variables (`let`, `mut`, types, shadowing, `const`)
- [ ] 03 — Functions
- [ ] 04 — Control flow
- [ ] 05 — Loops
- [ ] 06 — Ownership & borrowing
- [ ] 07 — Structs & enums
- [ ] 08 — Error handling

## Why this exists

I'm a Python developer learning Rust. I keep forgetting Rust rules because Python lets me get away with everything. This repo is my cheat sheet — one lesson at a time, Python first, Rust second, side by side.
