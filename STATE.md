# Project State — Resume Point

> **Read this first next time** to pick up exactly where we stopped.
>
> For full project context (architecture, conventions, resources, roadmap), see [`AGENTS.md`](./AGENTS.md).

**Last updated:** 21st June, 2026

---

## ✅ Fully done (committed + pushed)

- **Lesson 01** — Hello World ✅ 5th June, 2026
- **Lesson 02** — Variables ✅ 5th June, 2026
- **Lesson 03** — Functions ✅ 6th June, 2026
- **Lesson 04** — If/else ✅ 6th June, 2026
- **Lesson 05** — Loops ✅ 7th June, 2026
- **Lesson 06** — Ownership & Borrowing ✅ 7th June, 2026
- **Lesson 07** — Structs & Enums ✅ 14th June, 2026
- **Lesson 08** — Error Handling ✅ 21st June, 2026

---

## Next up

**Lesson 09 — Strings**: `String` vs `&str`, `format!`, `to_string()`, `to_owned()`, string slices, concatenation, common operations. Builds on ownership (borrowing strings) and error handling (parsing strings into numbers).

---

## Lesson 08 committed state

- `python/08_error_handling.py` — 10 sections: panic vs exceptions, Result pattern, unwrap/expect, ? operator simulation, error propagation chain, custom error types, map_err, main() returning Result, panic! vs Result heuristic, the Rust mindset
- `rust/08_error_handling.rs` — 10 sections: panic!, Result enum, unwrap/expect, ? operator, error propagation, unwrap strategies, custom error enum with Display/Error/From, map_err, main() returning Result, panic! vs Result
- `playground/src/bin/08_error_handling.rs` — cargo bin copy (clean compile)
- `playground/Cargo.toml` — `[[bin]] name = "08_error_handling"`
- `notes/08_error_handling-quickref.md` — one-pager (mental shift, Result api, ? operator, custom errors, From impls, unwrap strategies, common compile errors)
- `notes/08_error_handling.qmd` — rendered to HTML, 14 sections:
  - §1: The big mental shift — throwing vs returning
  - §2: panic! — the "everything is broken" button
  - §3: Result<T, E> — the enum that rules error handling
  - §4: unwrap() and expect() — quick extraction
  - §5: Graceful handling — the full toolkit
  - §6: The ? operator — Rust's killer feature
  - §7: Error propagation — chaining fallible functions
  - §8: Custom error types — the enum approach
  - §9: map_err() — inline error conversion
  - §10: Using ? in main()
  - §11: panic! vs Result — decision framework
  - §12: Walkthrough of 6 rustlings exercises (errors1–6)
  - §13: 5 "Try it yourself" exercises
  - §14: What we'll build on in lesson 09
- `README.md` — lesson 08 marked done
- `AGENTS.md` — progress table updated

(End of file - total 51 lines)
