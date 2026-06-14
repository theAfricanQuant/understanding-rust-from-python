/// Lesson 07 — Structs & Enums
/// Rust side: structs (named, tuple, unit), impl blocks, enums (ADTs),
/// Option<T>, match, if let, and ownership in struct fields.
///
/// If ownership (lesson 06) is "the borrow checker",
/// then structs & enums are "how you model your data".
/// Together they form the backbone of every Rust program.

/// ---- SECTION 1: Struct definitions ----
/// Python: @dataclass
/// Rust:   struct Name { field: Type, ... }

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn demo_struct() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("  {alice:?}");
    println!("  name={}, age={}", alice.name, alice.age);

    // You can't reassign fields unless the binding is `mut`
    let mut bob = Person {
        name: String::from("Bob"),
        age: 25,
    };
    bob.age = 26;
    println!("  bob after mutation: {bob:?}");
}

/// ---- SECTION 2: impl blocks (methods) ----
/// Python: methods inside the class body
/// Rust:   methods in a SEPARATE `impl` block
/// This separation is deliberate — you can have multiple impl blocks

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // &self = borrowed read access (like Python's `self`)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // No &self = associated function (like Python's @classmethod)
    // Called as Rectangle::square(5.0)
    fn square(side: f64) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn demo_methods() {
    let rect = Rectangle { width: 30.0, height: 50.0 };
    println!("  rect: {rect:?}");
    println!("  area: {}", rect.area());

    let smaller = Rectangle { width: 10.0, height: 20.0 };
    println!("  can hold smaller: {}", rect.can_hold(&smaller));

    let sq = Rectangle::square(5.0);
    println!("  square: {sq:?}");
}

/// ---- SECTION 3: Tuple structs ----
/// Python: NamedTuple
/// Rust:   struct Name(Type1, Type2);
/// Fields accessed by index: .0, .1, .2 — NO named fields!

#[derive(Debug)]
struct Color(u8, u8, u8); // (red, green, blue)

#[derive(Debug)]
struct Point(i32, i32); // (x, y)

fn demo_tuple_struct() {
    let green = Color(0, 255, 0);
    println!("  Color: {green:?}");
    println!("  red={}, green={}, blue={}", green.0, green.1, green.2);

    let origin = Point(0, 0);
    println!("  Point: {origin:?}, x={}, y={}", origin.0, origin.1);

    // Destructuring tuple structs
    let Color(r, g, b) = green;
    println!("  destructured: r={r}, g={g}, b={b}");
}

/// ---- SECTION 4: Unit structs ----
/// Python: a class with `pass` and no fields
/// Rust:   struct Name;
/// Used as type-level markers, phantom types, compile-time guards

#[derive(Debug)]
struct UnitMarker;

fn demo_unit_struct() {
    let marker = UnitMarker;
    println!("  unit struct: {marker:?}");
    // No data, no fields — just a type.
    // Useful for generics, traits, and compile-time state.
}

/// ---- SECTION 5: Struct update syntax ----
/// Python: dataclass.replace(template, field=value)
/// Rust:   Struct { field: new_value, ..template }
/// This MOVES heap fields from template (unless they implement Copy)
/// So you might not be able to use template afterwards!

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_email: bool,
    count: u32,
}

fn demo_update_syntax() {
    let template = Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_email: true,
        count: 0,
    };

    let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,
        ..template // ← copy the rest; name/count were already set
    };
    println!("  your_order: {your_order:?}");

    // println!("{template:?}");  // ❌ ERROR: template.name was MOVED (String)
    // If ALL fields were Copy types, template would still be usable.
    // With non-Copy fields, the moved fields make the struct partially invalid.
}

/// ---- SECTION 6: Enums (ADTs — Algebraic Data Types) ----
/// Python: Enum = named constants only
/// Rust:   enum = a type that can be ONE of several variants,
///         and each variant can hold DIFFERENT data.
/// This is one of Rust's most powerful features!

#[derive(Debug)]
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },       // named fields
    Write(String),                 // unnamed single field (tuple variant)
    ChangeColor(u8, u8, u8),       // unnamed multiple fields
}

impl Message {
    fn describe(&self) {
        match self {
            Message::Quit => println!("  Quit — no data"),
            Message::Move { x, y } => println!("  Move to ({x}, {y})"),
            Message::Write(text) => println!("  Write: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("  Change color to RGB({r}, {g}, {b})");
            }
        }
    }
}

fn demo_enums() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 128),
    ];

    for msg in &messages {
        msg.describe();
    }
}

/// ---- SECTION 7: match — exhaustive pattern matching ----
/// Python 3.10+ has match, but Rust's match is MUCH stricter:
/// - MUST cover ALL variants (the compiler checks exhaustiveness)
/// - Each arm is an expression (returns a value)
/// - Variables from patterns are BINDINGS, not comparisons

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("  Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("  Quarter from {state:?}!");
            25
        }
    }
    // If you forget a variant, the compiler says:
    // error[E0004]: non-exhaustive patterns: `Quarter(_)` not covered
}

fn demo_match() {
    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::California),
    ];
    for coin in &coins {
        let cents = value_in_cents(coin);
        println!("  {coin:?} = {cents} cents");
    }
}

/// ---- SECTION 8: Option<T> — Rust's replacement for null ----
/// Python: None (a singleton, everything can be None)
/// Rust:   Option<T> is an enum: Some(T) | None
/// The compiler FORCES you to handle both cases.
/// No more "NoneType has no attribute..." at runtime!

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn demo_option() {
    let result = divide(10.0, 2.0);
    match result {
        Some(v) => println!("  10/2 = {v}"),
        None => println!("  10/2 = undefined"),
    }

    let result = divide(10.0, 0.0);
    match result {
        Some(v) => println!("  10/0 = {v}"),
        None => println!("  10/0 = undefined"),
    }

    // Option has many useful methods:
    let x: Option<i32> = Some(5);
    println!("  unwrap_or: {}", x.unwrap_or(0)); // 5
    let y: Option<i32> = None;
    println!("  unwrap_or: {}", y.unwrap_or(0)); // 0

    // .unwrap() panics on None — use sparingly
    // .expect("msg") panics with a message on None
    println!("  expect: {}", x.expect("should be Some"));
    // println!("{}", y.expect("oops")); // panics: "oops"
}

/// ---- SECTION 9: if let — match for one pattern ----
/// Syntactic sugar when you only care about one variant.

fn demo_if_let() {
    let config_max: Option<u8> = Some(3u8);

    // Verbose match:
    match config_max {
        Some(max) => println!("  match: max = {max}"),
        _ => (), // do nothing
    }

    // if let — cleaner:
    if let Some(max) = config_max {
        println!("  if let: max = {max}");
    }

    // if let with else:
    let no_value: Option<u8> = None;
    if let Some(v) = no_value {
        println!("  got {v}");
    } else {
        println!("  got None");
    }

    // Works with any enum, not just Option:
    enum Status {
        Connected(String),
        Disconnected,
    }
    let s = Status::Connected(String::from("localhost:8080"));
    if let Status::Connected(addr) = s {
        println!("  status: connected to {addr}");
    }
}

/// ---- SECTION 10: Ownership inside structs and enums ----
/// When a struct/enum has heap-allocated fields (String, Vec<T>):
/// - Moving the struct moves ALL its heap data
/// - Dropping the struct frees ALL its heap data
/// - A field's ownership is separate from the struct's borrow status

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn print_config(config: &Config) {
    // Borrowing — no move
    println!("  {config:?}");
}

fn take_config(config: Config) {
    // Takes ownership — config is moved, will be dropped here
    println!("  consumed: {config:?}");
}

fn demo_struct_ownership() {
    let cfg = Config {
        host: String::from("localhost"),
        port: 8080,
    };

    print_config(&cfg);  // borrow — cfg still alive
    print_config(&cfg);  // borrow again — fine

    take_config(cfg);    // move — cfg is gone
    // print_config(&cfg); // ❌ ERROR: cfg was moved
    // println!("{cfg:?}"); // ❌ ERROR: use of moved value

    // MAJOR insight: you can't partially move fields.
    // let host = cfg.host;  // moves cfg.host → makes cfg partially invalid
    // println!("{cfg.port}"); // ERROR: can't use cfg after partial move
}

fn main() {
    println!("=== 1. Structs ===");
    demo_struct();
    println!("\n=== 2. impl (methods) ===");
    demo_methods();
    println!("\n=== 3. Tuple structs ===");
    demo_tuple_struct();
    println!("\n=== 4. Unit structs ===");
    demo_unit_struct();
    println!("\n=== 5. Struct update syntax ===");
    demo_update_syntax();
    println!("\n=== 6. Enums (ADTs) ===");
    demo_enums();
    println!("\n=== 7. match ===");
    demo_match();
    println!("\n=== 8. Option<T> ===");
    demo_option();
    println!("\n=== 9. if let ===");
    demo_if_let();
    println!("\n=== 10. Ownership in structs ===");
    demo_struct_ownership();
}
