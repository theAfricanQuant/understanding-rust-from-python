# AGENTS.md

> **Read this first** when starting work on this project (human or AI).
> It explains *what this project is*, *how it's organized*, *what the conventions are*, and *where to find things*.

---

## What this project is

A personal learning journal: **learning Rust from a Pythonista's perspective**, with side-by-side lessons.

- **Owner:** Ricky Macharm (`theAfricanQuant` on GitHub)
- **Repo:** https://github.com/theAfricanQuant/rust-from-python
- **Inspiration:** [rustlings](https://github.com/rust-lang/rustlings) exercises, augmented with Python baselines and "into the weeds" commentary for someone coming from a dynamic-typing background.
- **Format:** Every lesson is a `.qmd` file rendered to standalone HTML for easy reading. Code lives in 3 parallel locations: standalone `.rs`/`.py` files, plus a multi-bin `playground/` cargo project.

---

## Current state at a glance

**Last updated:** 7th June, 2026

| Lesson | Topic | Status | Date completed |
|---|---|---|---|
| 01 | Hello World | ✅ done | 5th June, 2026 |
| 02 | Variables | ✅ done | 5th June, 2026 |
| 03 | Functions | ✅ done | 6th June, 2026 |
| 04 | If/else | ✅ done | 6th June, 2026 |
| 05 | Loops | ✅ done | 7th June, 2026 |
| 06 | Ownership & borrowing | ⬜ pending | — |
| 07 | Structs & enums | ⬜ pending | — |
| 08 | Error handling | ⬜ pending | — |

**Detailed resume point** for any in-progress lesson: read `STATE.md` at the project root.

---

## Architecture — where things live

```
rust-from-python/
├── AGENTS.md                   ← this file
├── STATE.md                    ← resume point for in-progress work
├── README.md                   ← project landing page (progress table)
├── SETUP.md                    ← first-time-clone instructions
├── pyproject.toml              ← Python deps (uv-managed)
├── uv.lock                     ← committed lockfile
│
├── (no combined doc — one .qmd per lesson in notes/, render to HTML separately)
│
├── **No Jupyter** — code is run via `cargo`/`uv run python`/`rustc`. The .qmd files render to plain HTML with no code execution.
├── python/                     ← standalone Python scripts, one per lesson
│   ├── 01_hello.py
│   ├── 02_vars.py
│   └── 03_functions.py
│
├── rust/                       ← standalone Rust files, compile with rustc
│   ├── 01_hello.rs
│   └── 03_functions.rs        (02_vars.rs not written; covered in cargo)
│
├── playground/                 ← cargo multi-bin project (preferred way to run Rust)
│   ├── Cargo.toml              ← one [[bin]] entry per lesson
│   └── src/bin/
│       ├── 01_hello.rs
│       ├── 02_vars.rs
│       └── 03_functions.rs
│
├── notes/                      ← verbose explainers, one .qmd per lesson + tooling
│   ├── 01_hello.qmd
│   ├── 02_vars.qmd
│   ├── 03_functions.qmd        ← 12 sections, "into the weeds"
│   ├── 01_hello-quickref.md    ← one-pager (markdown, not rendered)
│   ├── 02_vars-quickref.md
│   └── running-rust.qmd        ← cargo vs rustc explained
│
├── .venv/                      ← uv-managed venv (gitignored, may be empty if no Python deps)
```

Rendered HTML files (`*.html`, `*_files/`) live next to their `.qmd` sources but are **gitignored**. Re-render locally.

---

## Conventions

### Per-lesson checklist (the 8-item pattern)

Every lesson produces these 8 deliverables, in this order:

| # | File | What |
|---|---|---|
| 1 | `python/XX_<topic>.py` | Standalone Python, heavily commented |
| 2 | `rust/XX_<topic>.rs` | Standalone Rust, compile with `rustc` |
| 3 | `playground/src/bin/XX_<topic>.rs` | Cargo bin version |
| 4 | `playground/Cargo.toml` | Add `[[bin]]` entry |
| 5 | `notes/XX_<topic>-quickref.md` | One-page cheatsheet (markdown) |
| 6 | `notes/XX_<topic>.qmd` | Verbose explainer (qmd → render to HTML) |
| 7 | `README.md` | Mark lesson done in progress table with date |
| 8 | Git commit + push | One commit: `"Lesson XX: <topic>"` |

> **Note (as of 6th June, 2026):** There is no combined `learn-rust.qmd` doc. Each lesson is its own self-contained `.qmd` in `notes/`. The user prefers this — chapter-by-chapter, not one mega-doc.

### Dates

- **Display format:** human-friendly, e.g. `5th June, 2026`
- **YAML frontmatter:** both formats, `date_completed: 2026-06-05` (ISO) + `date_display: "5th June, 2026"` (display)

### Python side

- **Package manager:** `uv` (the modern one)
- **Run any Python command with:** `uv run python ...`
- **Render qmd:** `uv run quarto render ...`
- **Virtualenv:** `.venv/` (auto-created by `uv sync`, gitignored, may be empty)
- **No Python runtime dependencies** — the project only uses the stdlib

### Rust side

- **Lesson files:** `cargo run --bin XX_<topic>` (preferred)
- **Quick experiments:** `rustc file.rs -o /tmp/x && /tmp/x`
- **Detailed tooling guide:** `notes/running-rust.qmd`

### Naming

- Rust functions/variables: `snake_case` (idiomatic)
- Rust types: `UpperCamelCase`
- Constants: `SCREAMING_SNAKE_CASE`
- File names: `XX_topic.ext` (zero-padded number prefix to keep alphabetical order)

---

## Tone & style of the lessons

The user is a **Pythonista** learning Rust. Every lesson should:

1. **Start with Python** (familiar), then show **Rust** (new)
2. **Highlight the mental shift** — what rule is different and why
3. **Use real compiler errors** as teaching moments
4. **Include 5+ rustlings exercises** walked through
5. **Be verbose** — the user explicitly asked to "get into the weeds"
6. **Have a Python parallel for every Rust concept**
7. **End with "Try it yourself"** — concrete exercises
8. **Cross-reference** related lessons (e.g. "we saw this in functions4")

The user reads the rendered HTML, not the qmd source. The .qmd file should produce a polished, scannable, bookmarkable HTML doc.

---

## Resources to consult

The user has bookmarked these Python→Rust resources. When writing or expanding a lesson, **read these for ideas and Python-parallel framings** before drafting from scratch.

| URL | What it covers |
|---|---|
| https://towardsdatascience.com/rust-for-python-developers-why-you-should-take-a-look-at-the-rust-programming-language/ | High-level "why Rust from Python" motivation, design philosophy |
| https://volzo.de/posts/rust-in-python/ | Calling Rust from Python (PyO3, maturin) — practical interop |
| https://blog.devgenius.io/learning-rust-as-a-second-language-from-a-pythonista-42eea6205b96 | Side-by-side Python/Rust, what to expect, gotchas |
| https://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/ | Armin Ronacher (Flask author) on Rust for Python devs — opinionated, design-focused |
| https://microsoft.github.io/RustTraining/python-book/ | **Microsoft's official Python→Rust training book** — likely the most comprehensive |
| https://doc.rust-lang.org/book/ | The Rust Book — the canonical reference |
| https://doc.rust-lang.org/rust-by-example/ | Rust by Example — code snippets, good for lookups |
| https://www.w3schools.com/rust/rust_functions.php | W3Schools — beginner-friendly tutorials (used in lesson 03) |

**Rule for agents:** Before drafting a new lesson, skim the relevant sections of these (especially Microsoft's book) to find Python parallels and good framings. Don't reinvent the wheel.

---

## Things to do next (priorities)

### Immediate (finish lesson 03)

- [ ] Write `notes/03_functions-quickref.md` (one-pager)
- [ ] Add lesson 03 section to `learn-rust.qmd` (side-by-side)
- [ ] Update `README.md` progress table — mark 03 done with `6th June, 2026`
- [ ] Final commit + push for lesson 03

### Short term

- [ ] Lesson 04: If/else (covers `match`, no truthy/falsy)
- [ ] Lesson 05: Loops (for/while/loop, break value trick)

### Medium term

- [ ] Lesson 06: Ownership & borrowing (the famous one)
- [ ] Lesson 07: Structs & enums
- [ ] Lesson 08: Error handling

### Long term / nice-to-have

- [ ] Build a small project at the end of each lesson (real code, not exercises)
- [ ] Add a "Common Python patterns in Rust" appendix — once enough lessons are done to see patterns
- [ ] Consider adding `examples/` to the cargo project — small standalone programs
- [ ] Add CI: GitHub Action to run `cargo test` + `uv run pytest` on every push

---

## Setup recap (what's installed)

| Tool | How installed | Used for |
|---|---|---|
| `rustc` 1.96.0, `cargo` 1.96.0, `rustup` 1.29.0 | rustup | Rust toolchain |
| `rust-analyzer` | rustup component | IDE support |
| `uv` 0.11.3 | astral.sh installer | Python deps (currently none) |
| `quarto` 1.9.37 | apt (.deb) | Render qmd to HTML (static, no execution) |
| `gcc`, `gdb`, `build-essential` | apt | Compilation + debugging |

**System:** TUXEDO OS 24.04 (Ubuntu base), Linux 6.17, x86_64.

---

## Open questions / decisions logged

- **qmd execution:** NONE. The .qmd files render to plain HTML with no code execution. Rust chunks are run via `cargo run --bin XX` from the `playground/` folder; Python chunks are run via `uv run python XX.py`. This is by design (decision: 6th June 2026, after user said "stop the qmd stuff" and "delete all the jupyter references").
- **Date format:** Human-friendly `5th June, 2026` for display, ISO `2026-06-05` in YAML. Both kept in `date_display` and `date_completed` fields.
- **Per-lesson 8-item pattern:** Established 6th June 2026 (was 9 items; dropped the combined `learn-rust.qmd` section). Stick with it.

---

## When you (the agent) start a new session

1. Read this file (AGENTS.md)
2. Read STATE.md for the in-progress lesson
3. Pick up where STATE.md leaves off
4. Maintain the same tone: verbose, Python-first, with weeds
5. Before drafting from scratch, skim the **Resources** URLs for relevant Python parallels
6. Commit + push after each significant chunk
7. Keep dates human-friendly
8. When in doubt, ask the user before doing something destructive
