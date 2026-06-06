# Project State — Resume Point

> **Read this first next time** to pick up exactly where we stopped.
>
> For full project context (architecture, conventions, resources, roadmap), see [`AGENTS.md`](./AGENTS.md).

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
| 5 | `notes/03_functions-quickref.md` | ✅ written — one-page cheatsheet, run commands, common errors |
| 6 | `notes/03_functions.qmd` (was `.md`) | ✅ written + rendered to HTML — 12 sections, into the weeds |
| 7 | `README.md` progress table | ✅ marked lesson 03 done with date `6th June, 2026` |

**Lesson 03 effectively complete (7/8 items).** Just needs the final commit.

---

## ❌ Still to do for lesson 03

| # | File | What it needs |
|---|---|---|
| ~~7~~ | ~~`learn-rust.qmd` side-by-side section~~ | **REMOVED** — user decided 6th June 2026 to drop the combined doc. Each lesson is its own self-contained `.qmd` in `notes/`. |
| 8 | `git commit` + `git push` | Final lesson 03 commit |

---

## Pattern change (6th June, 2026)

**Old per-lesson pattern (9 items):** ... included adding a section to `learn-rust.qmd`.
**New per-lesson pattern (8 items):** drop the combined doc. Just one `.qmd` per lesson in `notes/`.

See `AGENTS.md` for the updated 8-item checklist.

---

## Resume command

Next time, the assistant should:
1. Read this file (STATE.md) AND AGENTS.md for full context
2. Continue with the lesson 03 final commit
3. Then move to lesson 04 (control flow: if/else + match)

---

## Lessons fully done (committed + pushed)

- **Lesson 01** — Hello World ✅ 5th June, 2026
- **Lesson 02** — Variables ✅ 5th June, 2026

## Other completed setup work

- Repo pushed to `git@github.com:theAfricanQuant/rust-from-python.git`
- `pyproject.toml` + `uv.lock` (currently no Python deps — stdlib only)
- `SETUP.md` (first-time clone instructions)
- `notes/running-rust.qmd` (`cargo` vs `rustc` explained with Python parallels)
- All 4 qmd files in `notes/` render to standalone HTML (no jupyter, no execution)

## Decisions made / open questions

- **No jupyter, no execution in qmd** (decision 6th June 2026). .qmd files render to plain HTML. Code is run via `cargo`/`uv run python`/`rustc`. Cleaner, faster, fewer moving parts.
- **No combined `learn-rust.qmd` doc** (decision 6th June 2026). Each lesson is its own self-contained `.qmd` in `notes/`.
- **Date format:** Human-friendly `5th June, 2026` for display, ISO `2026-06-05` in YAML. Both kept in `date_display` and `date_completed` fields.
- **Per-lesson update pattern (8 files + commit + push):** Established and used for lessons 01, 02, 03.

## Out of scope (mention only if user asks)

- `play/` folder at parent level (old, single-bin cargo project — can be deleted any time)
- `rustlings/` folder at parent level (cloned rustlings exercises, used as reference for the lesson content, not run anymore)
