---
topic: "Hello World"
date_completed: 2026-06-05
status: done
rustlings_source: "exercises/00_intro/intro1.rs, intro2.rs"
---

# Lesson 01 — Hello World

| Python | Rust |
|---|---|
| `print("Hello")` | `println!("Hello");` |
| runs directly with `python file.py` | needs `fn main() { ... }` wrapper |

## Key idea
Rust **requires** an entry point called `main`. Python just runs top-to-bottom.

The `!` in `println!` means it's a **macro**, not a function. (You'll learn why later; for now just remember println has a bang.)

## Run it
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/01_hello.py
cd playground && cargo run --bin 01_hello
```
