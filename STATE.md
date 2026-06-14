# Project State — Resume Point

> **Read this first next time** to pick up exactly where we stopped.
>
> For full project context (architecture, conventions, resources, roadmap), see [`AGENTS.md`](./AGENTS.md).

**Last updated:** 14th June, 2026

---

## ✅ Fully done (committed + pushed)

- **Lesson 01** — Hello World ✅ 5th June, 2026
- **Lesson 02** — Variables ✅ 5th June, 2026
- **Lesson 03** — Functions ✅ 6th June, 2026
- **Lesson 04** — If/else ✅ 6th June, 2026
- **Lesson 05** — Loops ✅ 7th June, 2026
- **Lesson 06** — Ownership & Borrowing ✅ 7th June, 2026
- **Lesson 07** — Structs & Enums ✅ 14th June, 2026

---

## Next up

**Lesson 08 — Error handling**: `Result<T, E>`, `?` operator, `panic!` vs `Result`, `unwrap`/`expect`, custom error types. Builds directly on enums (`Result` is an enum just like `Option`).

---

## Lesson 07 committed state

- `python/07_structs_enums.py` — 9 sections: dataclass structs, methods, NamedTuple tuple structs, unit struct, Python enums vs ADT enums, `isinstance` dispatcher, `Optional`, `match`, ownership in structs
- `rust/07_structs_enums.rs` — 10 sections: structs, impl blocks, tuple structs, unit structs, update syntax, enums (ADTs), match, Option\<T\>, if let, ownership in structs
- `playground/src/bin/07_structs_enums.rs` — cargo bin copy (clean compile)
- `playground/Cargo.toml` — `[[bin]] name = "07_structs_enums"`
- `notes/07_structs_enums-quickref.md` — one-pager (3 struct kinds, impl signatures, enum variant shapes, match, Option methods, if let, ownership traps, common errors)
- `notes/07_structs_enums.qmd` — rendered to HTML, 13 sections:
  - §1: Python struct warm-up, `mut` and the compiler error
  - §2: 3 struct kinds (named, tuple, unit)
  - §3: Struct update syntax and the ownership trap
  - §4: Methods with `impl` blocks, `self` parameter cheat sheet
  - §5: Enums as algebraic data types — the "aha!" moment
  - §6: `match` — exhaustive, expression-based, binding vs comparison
  - §7: `Option<T>` — the enum you'll use everywhere
  - §8: `if let` — match for one pattern
  - §9: Ownership inside structs/enums
  - §10: The big picture — why enums are so important in Rust
  - §11: Walkthrough of 6 rustlings exercises (structs1–3, enums1–3)
  - §12: 5 "Try it yourself" exercises
  - §13: What we'll build on in lesson 08
- `README.md` — lesson 07 done
- `AGENTS.md` — progress table updated
