/// Lesson 07 — Structs & Enums (cargo version)
/// Run: cargo run --bin 07_structs_enums

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
    let mut bob = Person {
        name: String::from("Bob"),
        age: 25,
    };
    bob.age = 26;
    println!("  bob after mutation: {bob:?}");
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: f64) -> Self {
        Self { width: side, height: side }
    }
}

fn demo_methods() {
    let rect = Rectangle { width: 30.0, height: 50.0 };
    println!("  rect: {rect:?}, area: {}", rect.area());
    let smaller = Rectangle { width: 10.0, height: 20.0 };
    println!("  can hold smaller: {}", rect.can_hold(&smaller));
    println!("  square: {:?}", Rectangle::square(5.0));
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(i32, i32);

fn demo_tuple_struct() {
    let green = Color(0, 255, 0);
    println!("  Color: {green:?}, r={}, g={}, b={}", green.0, green.1, green.2);
    let origin = Point(0, 0);
    println!("  Point: {origin:?}, x={}, y={}", origin.0, origin.1);
    let Color(r, g, b) = green;
    println!("  destructured: r={r}, g={g}, b={b}");
}

#[derive(Debug)]
struct UnitMarker;

fn demo_unit_struct() {
    let marker = UnitMarker;
    println!("  unit struct: {marker:?}");
}

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
        ..template
    };
    println!("  your_order: {your_order:?}");
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) {
        match self {
            Message::Quit => println!("  Quit — no data"),
            Message::Move { x, y } => println!("  Move to ({x}, {y})"),
            Message::Write(text) => println!("  Write: {text}"),
            Message::ChangeColor(r, g, b) => println!("  Change color to RGB({r}, {g}, {b})"),
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

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
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
    let x: Option<i32> = Some(5);
    println!("  unwrap_or: {}", x.unwrap_or(0));
    println!("  expect: {}", x.expect("should be Some"));
}

fn demo_if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("  match: max = {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("  if let: max = {max}");
    }
    let no_value: Option<u8> = None;
    if let Some(v) = no_value {
        println!("  got {v}");
    } else {
        println!("  got None");
    }
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn print_config(config: &Config) {
    println!("  {config:?}");
}

fn take_config(config: Config) {
    println!("  consumed: {config:?}");
}

fn demo_struct_ownership() {
    let cfg = Config {
        host: String::from("localhost"),
        port: 8080,
    };
    print_config(&cfg);
    print_config(&cfg);
    take_config(cfg);
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
