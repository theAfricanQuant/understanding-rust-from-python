"""Lesson 07 — Structs & Enums
Python side: demonstrating classes, dataclasses, enums, and pattern matching.
Rust's structs/enums are similar in shape but VERY different in ownership rules."""

from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Optional


# ============================================================
# SECTION 1: Structs — Python @dataclass vs Rust struct
# ============================================================
# Python: @dataclass creates a struct-like thing with auto __init__, __repr__, etc.
# Rust:   struct Person { name: String, age: u8 }  — no methods by default
#         Methods are added separately via `impl` blocks

@dataclass
class Person:
    name: str
    age: int


def demo_python_struct():
    """Python: dataclass ≈ Rust struct."""
    alice = Person(name="Alice", age=30)
    print(f"  {alice}")
    print(f"  name={alice.name}, age={alice.age}")

    # Python: you can freely reassign fields
    alice.age = 31
    print(f"  after mutation: {alice}")

    # Rust: you'd need `let mut alice = Person { ... };` to mutate
    # Rust: alice.age = 31;  // OK because we declared `mut alice`


# ============================================================
# SECTION 2: Methods on structs — Python class vs Rust impl
# ============================================================
# Python: methods live inside the class body
# Rust:   methods live in a SEPARATE `impl` block

@dataclass
class Rectangle:
    width: float
    height: float

    def area(self) -> float:
        return self.width * self.height

    def can_hold(self, other: "Rectangle") -> bool:
        return self.width > other.width and self.height > other.height

    @classmethod
    def square(cls, side: float) -> "Rectangle":
        """Rust equivalent: associated function (no &self). Called as Rectangle::square(5.0)"""
        return cls(width=side, height=side)


def demo_methods():
    """Python: methods on dataclass"""
    rect = Rectangle(30.0, 50.0)
    print(f"  rect: {rect}")
    print(f"  area: {rect.area()}")

    smaller = Rectangle(10.0, 20.0)
    print(f"  can hold smaller: {rect.can_hold(smaller)}")

    sq = Rectangle.square(5.0)
    print(f"  square (classmethod): {sq}")


# ============================================================
# SECTION 3: Tuple structs
# ============================================================
# Python: NamedTuple or plain tuple
# Rust:   struct Color(u8, u8, u8);
#         Access fields by index: color.0, color.1, color.2

from typing import NamedTuple


class ColorTuple(NamedTuple):
    red: int
    green: int
    blue: int


def demo_tuple_struct():
    """Python NamedTuple ≈ Rust tuple struct."""
    green = ColorTuple(0, 255, 0)
    print(f"  NamedTuple: {green}")
    print(f"  by index: green[0]={green[0]}, green[1]={green[1]}, green[2]={green[2]}")
    print(f"  by name:  green.red={green.red}, green.green={green.green}, green.blue={green.blue}")
    # Rust tuple struct: you can ONLY access by .0, .1, .2 — no named fields!
    # In Python, NamedTuple gives both index AND name access.


# ============================================================
# SECTION 4: Unit-like struct (Rust: struct UnitStruct;)
# ============================================================
# Python: there's no exact equivalent.
# Rust:   struct UnitMarker; — a type with no fields, used as a marker/tag.
# In Python you'd use `None` or a sentinel object, but there's no type-level distinction.

class UnitMarker:
    """Approximating Rust's unit struct. No instance state."""
    pass


def demo_unit_struct():
    """Python: a class with no fields ≈ Rust unit struct."""
    marker = UnitMarker()
    print(f"  unit struct-ish: {marker}")
    # Rust: let m = UnitMarker;  (no braces, no parentheses)
    # Hands-on exercises
    # Rust: you can derive traits on it: #[derive(Debug, Clone, Copy)] struct UnitMarker;
    # Python: you'd implement __repr__, __eq__ manually


# ============================================================
# SECTION 5: Enums — Python Enum vs Rust enum
# ============================================================
# Python: Enum is a set of named constants. Variants CAN'T hold data.
# Rust:   enums are ALGEBRAIC DATA TYPES — each variant can hold different data!
#         This is one of Rust's most powerful features.


class Color(Enum):
    RED = auto()
    GREEN = auto()
    BLUE = auto()


def demo_python_enum():
    """Python: basic enum — just named constants."""
    c = Color.RED
    print(f"  color: {c}, value: {c.value}")

    # Python's Enum can't store extra data per variant.
    # If you need data, you hack it with __init__ or use a separate data class.
    # Rust: enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }
    #       Each variant can hold completely different types and amounts of data!


# ============================================================
# SECTION 6: Simulating Rust-style enums in Python
# ============================================================
# Rust: enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }
# Python: we use inheritance + isinstance() to fake it.
# This is CLUNKY in Python — Rust's match expression makes it elegant.

@dataclass
class MessageMove:
    x: int
    y: int


class Message:
    """Simulating Rust's enum with a class hierarchy."""
    pass


@dataclass
class Move(Message):
    x: int
    y: int


@dataclass
class Write(Message):
    text: str


class Quit(Message):
    pass


@dataclass
class ChangeColor(Message):
    red: int
    green: int
    blue: int


def process_message(msg: Message):
    """Python: messy isinstance chain. Rust: elegant match expression."""
    if isinstance(msg, Move):
        print(f"  Move to ({msg.x}, {msg.y})")
    elif isinstance(msg, Write):
        print(f"  Write: {msg.text}")
    elif isinstance(msg, Quit):
        print(f"  Quit")
    elif isinstance(msg, ChangeColor):
        print(f"  Color: ({msg.red}, {msg.green}, {msg.blue})")
    else:
        print(f"  Unknown message type")

    # Rust equivalent (much cleaner):
    # match msg {
    #     Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    #     Message::Write(text) => println!("Write: {}", text),
    #     Message::Quit => println!("Quit"),
    #     Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    # }


def demo_enum_as_adt():
    """Simulating Rust's algebraic data types in Python."""
    messages: list[Message] = [
        Move(x=10, y=20),
        Write(text="hello".encode().decode()),  # str
        ChangeColor(255, 0, 128),
        Quit(),
    ]
    for msg in messages:
        process_message(msg)


# ============================================================
# SECTION 7: Option<T> — Python None vs Rust's built-in enum
# ============================================================
# Python: None is a singleton. ANY variable can be None.
# Rust:   Option<T> is an enum with two variants: Some(T) or None.
#         The compiler FORCES you to handle the None case!


def divide(a: float, b: float) -> Optional[float]:
    """Python: return None on error. Caller might forget to check."""
    if b == 0:
        return None
    return a / b


def demo_option():
    """Python: Optional[T] vs Rust: Option<T>."""
    result = divide(10, 2)
    print(f"  10/2 = {result}")

    result = divide(10, 0)
    print(f"  10/0 = {result}")

    # Python: you can IGNORE the None — it'll crash at runtime
    # Rust:   the compiler WON'T LET YOU use an Option<T> without handling both cases.
    #         let result: Option<f64> = divide(10.0, 2.0);
    #         // result + 1.0;  // ❌ ERROR: cannot add to Option
    #         match result {
    #             Some(v) => println!("{v}"),
    #             None => println!("Cannot divide by zero"),
    #         }

    # Python 3.10+ has match: think of it as a pattern match
    match result:
        case None:
            print("  matched: no result")
        case float(value):
            print(f"  matched: {value}")
        case _:
            print("  unexpected")

    # Rust match is far more powerful — exhaustive, binding, guard clauses


# ============================================================
# SECTION 8: if let — just Python's match with a single case
# ============================================================
# Python: match statement with one arm
# Rust:   if let Some(v) = optional_value { ... }
#         Shorthand for match with one pattern + a catch-all else


def demo_if_let():
    """Python match ≈ Rust if let."""
    maybe_name: Optional[str] = "Alice"

    match maybe_name:
        case str(name):
            print(f"  matched name: {name}")
        case None:
            print(f"  no name")

    # Python equivalent of Rust's if let:
    # if let Some(name) = maybe_name {
    #     println!("{name}");
    # } else {
    #     println!("no name");
    # }


# ============================================================
# SECTION 9: Ownership in structs
# ============================================================
# Rust: structs own their heap-allocated data (String, Vec, etc.).
#       When a struct is moved, all its heap data moves with it.
#       When a struct is dropped, all its heap data is freed.
# Python: a dataclass just holds references. GC cleans up.


@dataclass
class Order:
    name: str
    year: int
    made_by_email: bool
    count: int


def demo_ownership_in_structs():
    """Python: assignment is a reference — both point to same Order."""
    template = Order(name="Bob", year=2019, made_by_email=True, count=0)

    # Python assignment = shared reference
    your_order = template
    your_order.name = "Hacker in Rust"  # mutates template too!
    your_order.count = 1

    print(f"  template: {template}")  # name changed!
    print(f"  your_order: {your_order}")

    # Rust struct update syntax:
    # let your_order = Order {
    #     name: String::from("Hacker in Rust"),
    #     count: 1,
    #     ..template  // copy the rest; template's heap fields are MOVED
    # };
    # // println!("{template:?}");  // ❌ ERROR if template had non-Copy heap fields

    # Python: you'd need Order(name="Hacker in Rust", year=template.year, ...)
    # or dataclass.replace(template, name="Hacker in Rust", count=1)


if __name__ == "__main__":
    print("=== 1. Python structs (dataclass) ===")
    demo_python_struct()
    print("\n=== 2. Methods on structs ===")
    demo_methods()
    print("\n=== 3. Tuple structs (NamedTuple) ===")
    demo_tuple_struct()
    print("\n=== 4. Unit structs ===")
    demo_unit_struct()
    print("\n=== 5. Python enums ===")
    demo_python_enum()
    print("\n=== 6. Simulating ADT enums in Python ===")
    demo_enum_as_adt()
    print("\n=== 7. Option<T> vs None ===")
    demo_option()
    print("\n=== 8. if let ===")
    demo_if_let()
    print("\n=== 9. Ownership in structs ===")
    demo_ownership_in_structs()
