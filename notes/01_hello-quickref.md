# Lesson 01 — Hello World

**Completed:** 5th June, 2026 · **Status:** done

Python equivalent lives in `../python/01_hello.py`.
Rust standalone (compile with `rustc`) lives in `../rust/01_hello.rs`.
Rust multi-bin (run with cargo) lives in `../playground/src/bin/01_hello.rs`.
Full verbose lesson with explanations: see `../../learn-rust.qmd` (lesson 01).

## Run it (3 ways)

**Python:**
```bash
python3 python/01_hello.py
```

**Rust standalone:**
```bash
rustc rust/01_hello.rs -o /tmp/01_hello && /tmp/01_hello
```

**Rust via cargo:**
```bash
cd playground && cargo run --bin 01_hello
```
