# Setup Guide — First-Time Clone

Everything you need to go from a fresh `git clone` to a working environment.

---

## 1. Prerequisites — install these once

You need four tools on your system. Check what's missing with:

```bash
rustc --version    # Rust compiler
cargo --version    # Rust package manager (comes with rustc)
python3 --version  # Python 3.11+
uv --version       # Modern Python package manager
quarto --version   # For rendering .qmd files
```

### Install what's missing

**Rust** (skip if `rustc --version` works):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# follow prompts, then:
source "$HOME/.cargo/env"
```

**uv** (skip if `uv --version` works):
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
source "$HOME/.local/bin/env"
```

**Python 3.11+** (TUXEDO/Ubuntu):
```bash
sudo apt update && sudo apt install python3 python3-venv python3-dev
```

**Quarto** (skip if `quarto --version` works):
```bash
# Download .deb from https://quarto.org/docs/get-started/
sudo apt install ./quarto-1.9.37-linux-amd64.deb
```

**Build essentials** (needed for some Python packages and Rust crates):
```bash
sudo apt install build-essential gcc g++ make
```

---

## 2. Clone the repo

```bash
git clone git@github.com:theAfricanQuant/rust-from-python.git
cd rust-from-python
```

---

## 3. Python side — uv handles everything

```bash
# creates .venv/ and installs every dep from pyproject.toml + uv.lock
uv sync

# verify it works
uv run python --version
uv run python -c "import jupyter, ipykernel, matplotlib, numpy; print('ok')"
```

**Every Python command, prefix with `uv run`:**
```bash
uv run python python/01_hello.py
uv run jupyter lab              # if you want a notebook UI
```

If `uv run` ever feels verbose, activate the venv once:
```bash
source .venv/bin/activate
python python/01_hello.py       # now plain `python` works
deactivate                      # when done
```

---

## 4. Rust side — cargo

```bash
# build the multi-bin playground once
cd playground
cargo build
cargo run --bin 01_hello
cargo run --bin 02_vars
cd ..
```

**Standalone `.rs` files (no cargo):**
```bash
rustc rust/01_hello.rs -o /tmp/01_hello
/tmp/01_hello
```

---

## 5. Render the side-by-side doc

```bash
quarto render learn-rust.qmd --to html
```

Open the result:
```bash
xdg-open learn-rust.html        # Linux
# or just open it in VS Code
```

---

## 6. (Optional) Make the `{rust}` chunks in the qmd actually run

The .qmd has Python chunks (always work) and Rust chunks (need an extra kernel).

```bash
# install evcxr (a Rust Jupyter kernel) — takes 5-10 min to compile
cargo install evcxr_jupyter
evcxr_jupyter --install

# verify
jupyter kernelspec list
# you should see: python3  rust
```

Now `quarto render learn-rust.qmd` will execute `{rust}` chunks too.

---

## 7. Verify everything

Run this checklist:

```bash
# Python
uv run python python/01_hello.py                # prints "Hello, Rust!"

# Rust cargo
cd playground && cargo run --bin 01_hello && cd ..   # prints same

# Rust standalone
rustc rust/01_hello.rs -o /tmp/h && /tmp/h       # prints same

# Quarto
quarto render learn-rust.qmd --to html          # produces learn-rust.html
```

If all four pass, you're set.

---

## Troubleshooting

| Problem | Fix |
|---|---|
| `uv: command not found` | `source ~/.local/bin/env` or restart terminal |
| `cargo: command not found` | `source "$HOME/.cargo/env"` or restart terminal |
| `linker 'cc' not found` | `sudo apt install build-essential` |
| `error: failed to run custom build command for openssl-sys` | `sudo apt install libssl-dev pkg-config` |
| Quarto chunk `{rust}` fails | install evcxr (section 6) |
| `Permission denied (publickey)` on git push | your SSH key isn't on the GitHub account — add `~/.ssh/id_ed25519.pub` to https://github.com/settings/keys |

---

## Where things go

```
rust-from-python/
├── pyproject.toml + uv.lock   ← Python deps (uv sync installs these)
├── learn-rust.qmd             ← the lesson document
├── python/XX.py               ← reference Python (uv run python XX.py)
├── rust/XX.rs                 ← standalone Rust (rustc XX.rs && ./XX)
├── playground/                ← multi-bin Rust (cargo run --bin XX)
│   ├── Cargo.toml
│   └── src/bin/XX.rs
├── notes/                     ← quick-reference markdown per lesson
└── .venv/                     ← uv venv, gitignored, recreated by `uv sync`
```

**Three patterns, three commands:**

| Pattern | Command |
|---|---|
| Python (uv) | `uv run python python/XX.py` |
| Rust standalone | `rustc rust/XX.rs -o /tmp/XX && /tmp/XX` |
| Rust cargo | `cd playground && cargo run --bin XX && cd ..` |
