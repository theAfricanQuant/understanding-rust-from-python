# Lesson 07 — Structs & Enums — Quick Reference

**Completed:** 14th June, 2026 · **Status:** done

Verbose explainer: `07_structs_enums.qmd`

---

## Structs: 3 kinds

| Kind | Syntax | Access | Python parallel |
|---|---|---|---|
| **Named fields** | `struct P { x: i32, y: i32 }` | `p.x`, `p.y` | `@dataclass` |
| **Tuple struct** | `struct Color(u8, u8, u8)` | `c.0`, `c.1`, `c.2` | `NamedTuple` |
| **Unit struct** | `struct Marker;` | (none — type only) | `class X: pass` |

### Instantiation

```rust
let p = Person { name: String::from("A"), age: 30 };
let green = Color(0, 255, 0);
let marker = UnitMarker;
```

### Update syntax (copy-with-override)

```rust
let order = Order { name: String::from("Me"), count: 1, ..template };
// WARNING: heap fields (String, Vec) are MOVED — template may be invalid after!
```

---

## impl blocks (methods)

Methods go in SEPARATE `impl` blocks — NOT inside the struct:

```rust
impl Rectangle {
    fn area(&self) -> f64        { self.width * self.height }  // method
    fn can_hold(&self, o: &Rect) -> bool { ... }               // method
    fn square(side: f64) -> Self { Self { w: side, h: side } } // associated fn
}
```

| `self` param | Called as | Can mutate fields? | Takes ownership? |
|---|---|---|---|
| `&self` | `rect.area()` | ❌ no | ❌ borrows |
| `&mut self` | `rect.grow()` | ✅ yes | ❌ borrows mutably |
| `self` | `shape.consume()` | ✅ yes | ✅ moves (rare) |
| *(no self)* | `Rect::square(5.0)` | N/A (no instance) | N/A |

---

## Enums (Algebraic Data Types)

The key insight: **Rust enums can hold DIFFERENT data per variant**. Python enums can't.

```rust
enum Message {
    Quit,                           // no data
    Move { x: i32, y: i32 },        // named fields
    Write(String),                  // unnamed single field
    ChangeColor(u8, u8, u8),        // unnamed multiple fields
}

let m = Message::Write(String::from("hi"));
```

### The 3 variant shapes

| Shape | Example | Destructure |
|---|---|---|
| No data | `Quit` | `Message::Quit =>` |
| Named fields | `Move { x: 10, y: 20 }` | `Message::Move { x, y } =>` |
| Unnamed (tuple) | `Write(String::from("hi"))` | `Message::Write(text) =>` |

---

## match (exhaustive pattern matching)

**The compiler checks you've covered EVERY variant.** If you miss one → compile error.

```rust
match coin {
    Coin::Penny      => 1,
    Coin::Nickel     => 5,
    Coin::Dime       => 10,
    Coin::Quarter(s) => 25,  // binds `s` to the inner state
}
// Forgetting a variant → error[E0004]: non-exhaustive patterns
```

`match` is an **expression** — each arm returns a value.

---

## Option<T> — Rust's null

`Option<T>` is just an enum: `Some(T) | None`

```rust
enum Option<T> { None, Some(T) }  // this is literally it (simplified)
```

| Operation | Returns | Panics? |
|---|---|---|
| `opt.unwrap()` | inner `T` | ✅ panics on `None` |
| `opt.expect("msg")` | inner `T` | ✅ panics on `None` with message |
| `opt.unwrap_or(default)` | inner `T` or `default` | ❌ never |
| `match opt { Some(v) => ..., None => ... }` | anything | ❌ never |

**Critical difference from Python:** The compiler forces you to handle `None`. You can't accidentally call `.method()` on a `None`.

---

## if let — match for one pattern

```rust
// Verbose:
match opt { Some(v) => println!("{v}"), _ => () }

// Clean:
if let Some(v) = opt {
    println!("{v}");
} else {
    // runs on any OTHER variant (not just None)
}
```

---

## Ownership in structs & enums

- **Moving** a struct/enum moves ALL its heap data (String, Vec)
- **Dropping** a struct/enum frees ALL its heap data
- **You can't partially move** — can't move out a single field unless the whole struct goes
- Structs with all-`Copy` fields are themselves `Copy` — can't derive it on structs with `String`, `Vec`, etc.

```rust
let name = person.name;  // ❌ partial move — person is now invalid
let Person { name, .. } = person;  // ✅ destructure move whole thing
```

### Struct update syntax ownership trap

```rust
let order2 = Order { name: String::from("Me"), ..order1 };
// order1.name was MOVED → order1 is partially invalid
// println!("{order1:?}");  // ❌
```

---

## Run it

**Python:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
uv run python python/07_structs_enums.py
```

**Rust via cargo (recommended):**
```bash
cd playground
cargo run --bin 07_structs_enums
```

**Rust standalone:**
```bash
cd /home/siseng/Documents/programing_languages/Rust/rust-from-python
rustc rust/07_structs_enums.rs -o /tmp/07_structs_enums && /tmp/07_structs_enums
```

---

## Common compile errors

| Error | Cause | Fix |
|---|---|---|
| `non-exhaustive patterns` | `match` missing a variant | add the variant or use `_ =>` wildcard |
| `cannot move out of borrowed content` | tried to take field from `&self` | clone or return a reference |
| `cannot move out of type X which implements Drop` | partial move from struct with `Drop` | destructure into pieces, or use `mem::take` / `clone` |
| `use of partially moved value` | used struct after moving a field out | redesign: consume the whole thing or clone |
| `no method named X found` | forgot `pub` on method, or wrong `self` param | add `pub`, check `&self`/`&mut self`/`self` |
| `struct literal body has no fields` | forgot `..` in update syntax or braces are empty | `Struct { field: val, ..other }` |

---

## Key mental models

- **struct = product type** (Rectangle has width AND height)
- **enum = sum type** (Message is Quit OR Move OR Write OR ...)
- **Rust enums are like sealed classes in Kotlin/Scala**, not like Python/C# enums
- **`match` is like Python's `match` statement (3.10+), but the compiler enforces exhaustiveness**
- **Methods live in `impl` blocks, not the struct body** — you can have multiple `impl` blocks
- **References to struct/enum fields obey the same borrow rules as everything else**
