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
cd /home/siseng/Documents/programing_languages/Rust/playground
cargo run --bin 01_hello
```

Or the Python version:
```bash
python3 python/01_hello.py
```
