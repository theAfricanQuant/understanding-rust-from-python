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

---

## Next up

**Lesson 06 — Ownership & borrowing**, the foundational Rust concept that explains everything else (why `for x in items` consumes, why `&` matters, why `break` with value is safe). This is the hardest lesson for Pythonistas — plan to spend the most time here.

---

## Per-lesson checklist (8 items)

| # | File | Status |
|---|---|---|
| 1 | `python/XX_topic.py` | ✅ |
| 2 | `rust/XX_topic.rs` | ✅ |
| 3 | `playground/src/bin/XX_topic.rs` | ✅ |
| 4 | `playground/Cargo.toml` [[bin]] | ✅ |
| 5 | `notes/XX_topic-quickref.md` | ✅ |
| 6 | `notes/XX_topic.qmd` (render to HTML) | ✅ |
| 7 | `README.md` progress table | ✅ |
| 8 | Git commit + push | ✅ |

---

## Other setup

- Repo pushed to `git@github.com:theAfricanQuant/rust-from-python.git`
- `pyproject.toml` + `uv.lock` (no Python deps — stdlib only)
- `SETUP.md` (first-time clone instructions)
- `notes/running-rust.qmd` (`cargo` vs `rustc` explained with Python parallels)
- All `.qmd` files render to standalone HTML (no jupyter, no execution)
- All `.html` and `_files/` directories gitignored; `*.pdf` gitignored
- `docs/speed_up_your_Python_with_Rust.md` tracked, `.pdf` version gitignored

---

## Committed state (lesson 05)

- `python/05_loops.py` — 9 functions, Python-only features (for..else) highlighted
- `rust/05_loops.rs` — 9 functions, Rust-only features (break with value, labels) highlighted
- `playground/src/bin/05_loops.rs` — cargo bin copy
- `playground/Cargo.toml` — has `[[bin]] name = "05_loops"`
- `notes/05_loops-quickref.md` — one-pager
- `notes/05_loops.qmd` — rendered to HTML, 13 sections with 2 stories:
  - §2: "The broken microwave" — why Rust has `loop` instead of `while True`
  - §3: "The delivery driver who needed a receipt" — `break` with a value
  - §6: Ownership fork — borrow vs consume in `for` loops
  - §8: Loop labels — `'outer: break 'outer` for nested loop breaks
- `README.md` — lesson 05 marked done, outdated references cleaned
- `AGENTS.md` — progress table + last-updated date updated
