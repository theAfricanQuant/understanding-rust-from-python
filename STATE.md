# Project State — Resume Point

> **Read this first next time** to pick up exactly where we stopped.
>
> For full project context (architecture, conventions, resources, roadmap), see [`AGENTS.md`](./AGENTS.md).

**Last updated:** 7th June, 2026

---

## ✅ Fully done (committed + pushed)

- **Lesson 01** — Hello World ✅ 5th June, 2026
- **Lesson 02** — Variables ✅ 5th June, 2026
- **Lesson 03** — Functions ✅ 6th June, 2026
- **Lesson 04** — If/else ✅ 6th June, 2026
- **Lesson 05** — Loops ✅ 7th June, 2026
- **Lesson 06** — Ownership & Borrowing ✅ 7th June, 2026

---

## Next up

**Lesson 07 — Structs & enums**: building custom types, `Option<T>`, and how ownership appears in struct fields. Structs are the foundation for the next lessons (error handling with `Result<T,E>`, pattern matching).

---

## Lesson 06 committed state

- `python/06_ownership.py` — 6 functions showing Python refcounting, shared mutation, scoping, copy vs assign, mutability contrast
- `rust/06_ownership.rs` — 8 functions: move, clone, copy, borrow (`&`), mut borrow (`&mut`), borrow checker, slices, no-dangle
- `playground/src/bin/06_ownership.rs` — cargo bin copy (clean compile, no warnings)
- `playground/Cargo.toml` — `[[bin]] name = "06_ownership"`
- `notes/06_ownership-quickref.md` — one-pager (3 ownership rules, Copy vs Move table, borrow types, common errors)
- `notes/06_ownership.qmd` — rendered to HTML, 15 sections, 3 stories:
  - §2: "The library book" — ownership, move, clone, drop on scope exit
  - §3: "The contractor with the only key" — borrowing & vs &mut, the golden rule
  - §4: "The Post-it vs the house" — Copy vs Move, stack vs heap
  - §12: 5 move_semantics exercises walkthrough
- `README.md` — lesson 06 done
- `AGENTS.md` — progress table updated
