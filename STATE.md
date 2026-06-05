# Project State — Resume Point

> **Read this first next time** to pick up exactly where we stopped.

**Last updated:** 5th June, 2026

---

## Currently in progress

**Lesson 03 — Functions** (started 5th June, 2026)

The user asked to "go into the weeds" — verbose explanations wanted, all 5 rustlings exercises covered, Python parallels throughout.

---

## ✅ Done for lesson 03

| # | File | Status |
|---|---|---|
| 1 | `python/03_functions.py` | ✅ written + tested, runs cleanly |
| 2 | `rust/03_functions.rs` | ✅ written + tested with `rustc` |
| 3 | `playground/src/bin/03_functions.rs` | ✅ written + tested with `cargo run --bin 03_functions` |
| 4 | `playground/Cargo.toml` | ✅ added `[[bin]]` entry for 03_functions |

**All three Rust/Python lesson files run and produce expected output.**

---

## ❌ Still to do for lesson 03

| # | File | What it needs |
|---|---|---|
| 5 | `notes/03_functions-quickref.md` | One-page Python↔Rust function cheatsheet, with run commands |
| 6 | `notes/03_functions.md` | Verbose lesson notes — go "into the weeds":<br>• All 5 rustlings exercises walked through<br>• Python parallels for each<br>• Implicit return (expression vs statement)<br>• Why types in params are mandatory<br>• Tuples for multiple return values<br>• No `*args`/`**kwargs` — use slices<br>• Function pointers (functions as values)<br>• Frontmatter: `topic`, `date_completed: 2026-06-05`, `date_display: "5th June, 2026"`, `status: done` |
| 7 | `learn-rust.qmd` | Add Lesson 03 section with the 4 main side-by-side chunks:<br>• Python vs Rust basic function<br>• Python vs Rust params + return<br>• Python vs Rust multiple returns (tuple)<br>• Python vs Rust implicit return demo |
| 8 | `README.md` | Update progress table — mark 03 done with date `5th June, 2026` |
| 9 | `git commit` + `git push` | One commit: `"Lesson 03: functions"` |

---

## Resume command

Next time, the assistant should:
1. Read this file
2. Continue with items #5–9 above
3. Maintain the same verbose style (user explicitly asked to "get into the weeds")

---

## Lessons fully done (committed + pushed)

- **Lesson 01** — Hello World ✅ 5th June, 2026
- **Lesson 02** — Variables ✅ 5th June, 2026

## Other completed setup work

- Repo pushed to `git@github.com:theAfricanQuant/rust-from-python.git`
- `pyproject.toml` + `uv.lock` (Python deps via `uv`)
- `SETUP.md` (first-time clone instructions)
- `notes/running-rust.md` (`cargo` vs `rustc` explained with Python parallels)
- `learn-rust.qmd` Python chunks execute via jupyter kernel (verified 3 outputs)
- Rust jupyter kernel installed (`evcxr_jupyter` — Rust chunks render as code only; Quarto jupyter is single-kernel, so Rust execution is via `cargo run --bin XX`)

## Decisions made / open questions

- **qmd Rust execution:** Deferred. User asked to stop the qmd/option-B discussion and focus on running Rust via `cargo`/`rustc`. State of the doc: Python chunks execute, Rust chunks render as code-only. Not revisited yet.
- **Date format:** Human-friendly `5th June, 2026` for display, ISO `2026-06-05` in YAML frontmatter. Both kept in `date_display` and `date_completed` fields.
- **Per-lesson update pattern (8 files + commit + push):** Established and used for lessons 01 and 02.

## Out of scope (mention only if user asks)

- `play/` folder at parent level (old, single-bin cargo project — can be deleted any time)
- `rustlings/` folder at parent level (cloned rustlings exercises, used as reference for the lesson content, not run anymore)
