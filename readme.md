# The Rust Programming Language

> Matterial is [https://doc.rust-lang.org/](https://doc.rust-lang.org/), Environment is macOS

# Introduction

- Rust, gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs
- Cargo, dependency manager and build tool
- Rustfmt, coding style
- The Rust Language Server for IDE

# Getting Started

## Installation

- Run `$ curl https://sh.rustup.rs -sSf | sh`
- Add the following line to your env `export PATH="$HOME/.cargo/bin:$PATH"`
- If installation is already done, run `rustup update`
  - To uninstall, `rustup self uninstall`

## Hello, World!

```rust
// main.rs
fn main() {
  println!("Hello, world!"); // `!` is macro. end with ';'
}

$ rustc main.rs
$ ./main
```

## Hello, Cargo

- Cargo is Rust's build system and package manager
- `cargo new $PROJECT_NAME —bin`
- TOML format using in Cargo.toml

```rust
  [package]
  name = "hello_cargo"
  version = "0.1.0"
  authors = ["Your Name <you@name.com>"]

  [dependencies]
```

- commands

```rust
# build
$ cargo build

# check before build
cargo check

# run
$ ./target/debug/hello_cargo
or
$ cargo run
```

# Programming a Guessing Game

- Create a new project and run

```
$ cargo new guessing_game --bin
$ cargo ru
```

- Standard library

```
use std::io
```

- Storing values with variables

```
let mut guess = String::new();
let foo = bar
let foo = 5; // immutable
let mut bar = 4; // mutable
```

- Handling potential failure
  - `Result` types are enumerations
  - without `except`, you will see warning

```rust
// using exepct for io::Result type
io::stdin().read_line(&mut guess)
​	.expect("Failed to read line");
```

- `{}` is placeholder to print variable

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

- Add dependencies
  - `cargo build` to install dependencies
  - `cargo update` to update dependencies

```
// in Cargo.toml
[dependencies]

rand = "0.3.14"
```

- Using dependencies

```
extern crate rand;
use rand:Rng;
```

- `match` expression

```
use std:cmp::Ordering;

match guess.cmp(&cecret_number) {
​   Ordering::Less => println!("Too small!"),
​   Ordering::Greater => println!("Too big!"),
​   Ordering::Equal => println!("Too win!"),
}
```

- Convert value

```
let guess: u32 = guess.trim().parse().expect("Error");
```

- Loop

```
loop {
}
```

- Quit the loop

```
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
      println!("You win!");
      break;
    }
}
```

- Handling invalid value

```
let guess: u32 = match guess.trim().parse() {
​    Ok(num) => num,
​    Err(_) => continue,
};
```

# Common Programming Concepts

## Variables and Mutability

- Default variables are immutable
- Using `mut` to mutable variable

## Differences Between Variable and Constants

- No `mut` for constants
- Constants can be declared in any scope
- Set with constant expression

## Shadowing

- Without `let` we will get compile error

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // (5 + 1) * 2 = 12
    println!("The value of x is: {}", x);
}
```

- We can change the type, reuse the same name

```rust
// reuse the variable with the another type
let spaces = "   "; // string type
let spaces = spaces.len(); // bind new type value

// compile error with using mut
let mut spaces = "   ";
spaces = spaces.len();
```

## Data Types

### Scalar Types

Single value: integers, floating-point numbers, Booleans, and characters

- Integers: i/u8, i/u16 ... i/u128, i/usize
  - Number literals:
    - Decimal: 98_222
    - Hex: 0xff
    - Octal: 0o77
    - Binary?: 0b1111_0000
    - Byte(u8): b'A'
    - u8: 57u8
  - Overflow, in debug, it panic, in production, wrapping
- Floating-Point: f64, default, same speeed as f32, double precision. f32, single precision
- Numberic Operations: `+, -, *, / , %`
- The Boolean Type: `true, false`
- The Character Type: `char`, single quotes. `string literal`, double quotes. unicode scalar values range from U+0000 to u+D7FFF and U+E000 to U+10FFFF

### Compound Types

Group multiple values into one type, tuple and arrays

- The Tuple Type, fixed length, once declared, cannot grow or shirnk in size

```rust
// decalration
let tup: (i32, f64, u8) = (500, 6.4, 1);

// access with index
let five_hundred = tup.0; // start with 0
let six_point_four = tup.1;
let one = tup.2;

// destructure
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

- The Array Type, collection of multiple values with the same type, fixed length unlike other languages.

```rust
// declaration
let a = [1, 2, 3, 4, 5];
let b: [i32; 5] = [1, 2, 3, 4, 5];

// accessing
let first = a[0];

// out of indexing, rust protects this kind of error from memory corruption
let index = 10;
let run_time_error = a[index];
```

## Functions

- Using snake case
- Defining function anywhere
- Parameters

```rust
fn another_function(x) {
  println!("The value of x is: {}", x);
}

fn another_function(y: i32) {
  println!("The value of y is: {}", y);
}
```

- Statements and Expressions

```rust
fn main() {
  let x = 5;
  let y = {
    let x = 3;
    x + 1 // no semicolun at the end of the expression
  }

  println!("y is {}", y); // y is 4
}
```

- Functions with Return Values

```rust
fn five() -> i32 {
  5 // end of expression without return
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn return_x(x: i32) -> i32 {
  if (x == 1) {
    return 0; // return keyword
  }

  x
}
```

## Comments

- Commenting with two slashes and continue until the end of line

## Control Flow

- if expression

```rust
let number = 3;
if number < 5 {
  println!("number is under 5");
} else {
   println!("number isnot under 5");
}

// error
if number {
  ...
}

// fix it
if number != 0 {
  ...
}

// else if
if number != 0 {
  ...
} else if number % 2 {
  ...
}

// if in a let statement
let condition = true;
let number = if confition {
  5 // must be same type returns
} else {
  6
}
```

- Repeatition with Loops

```rust
fn main() {
  let mut counter = 0;
  let result = loop {
    counter += 1;

    println!("again!");

    if counter == 10 {
      break counter * 2
    }
  }
}
```

- Contitional Loops with while

```rust
fn main() {
  let mut number = 3;
  while number != 0 {
    println!("again!");
    number = number - 1;
  }
}
```

- Looping Through a Collection with for

```rust
fn main() {
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("val is {}", element);
  }

  for number in (1..4).rev() {
    println!("val is {}", number);
  }
}
```

# Understanding Ownership

- To enables Rust to make memory safety guarantees without needing a garbage collector.
- In Ruet, memory is managed through a system of ownership with a set of rules that compiler checks at compile time

## What is Ownership?

### Ownership Rules

- Each value in Rust has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dopped

### Variable Scope

```rust
{ // s is not valid here, it's not yet declared
  let s = "hello";
} // this scope is over, and s is no longer valid
```

### The String Type

```rust
let s = String::from("hello"); // string is allocated on the heap

let mut s2 = String::from("hello");
s2.push_str(", world!");
```

### Memory and Allocation

```rust
{
  let s = String::from("hello"); // s is valid from this point forward
  ...
} // this scope is now over, s goes out of scope, and s is no longer valid
```

### Ways Variables and Data Interact: Move

```rust
let s1 = String::from("hello"); // s1 has string data(ptr, len, capacity)

let s2 = s1; // s1 string data is moved to s2, Rust invalidate s1

println!("{}, world!", s1); // error, string data has been moved to s2
```

### Ways Variables and Data Interact: Clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### Stack-Only Data: Copy

```rust
let x = 5;
let y = x; // x is stil valid after x moved to y

println!("x = {}, y = {}", x, y)
```

#### Copy rules

- All the integer types, such as `u32`
- The Boolean type `bool`, with values `true` and `false`
- All the floating point type, such as `f64`
- The character type `char`
- Tuples, if they only contain types that are also `Copy`. For example, `(i32, i32)` is `Copy` but `(i32, string)` is not

### Ownership and Function

- Passing a variable to a function will `move or copy`, just as assgiment does

```rust
fn main() {
  let s = String::from("hello"); // s comes into scope

  // s's value moves into the function
  // s is no longer valid
  takes_ownership(s);

  let x = 5; // x comes into scope

  // x would move into the function
  // but i32 is Copy, so it's ok to use x afterward
  makes_copy(x);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // some_string goes out of scope and drop, `the backing memory is freed`

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // some_integer goes out of scope. `Nothing special happens.`
```

### Return Values and Scope

- Returning values can also transfer ownership

```rust
fn main() {
  // gives_ownership moves its return value into s1
  let s1 = gives_ownership();

  // s2 comes into scope
  let s2 = String::from("hello");

  // 1. s2 is moved into takes_and_gives_back
  // 2. takes_and_gives_back moves its return value into s3
  let s3 = takes_and_gives_back(s2);
} // s3 goes out of scope, and is dropped
  // s2 goes out of scope, but was moved, so nothing happens
  // s1 goes out of scope, and it dropped

fn gives_ownership() -> String {
  let some_string = String::from("hello");

  // some_string is returned and moves out to the calling function
  some_string
}

fn takes_and_givs_back(a_string: String) -> String {
  // a_string is returned and moves out to the calling function
  a_string
}
```

### Returning with Tuple

```rust
fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
```

## References and Borrowing

```rust
fn main() {
  let s1 = String::from("hello");
  // reference, it does not own it
  // s1 will not be dropped when the reference goes out of scope
  let len = calculate_length(&s1);

  println!("{}, {}", s1, len);
}

fn calulate_length(s: &String) -> usize {
  // https://doc.rust-lang.org/book/img/trpl04-05.svg
  s.len()
}
```

### Mutable References

```rust
fn main() {
  let mut s = String::from("hello");

  change(&mut s);
  cannot_change(&s);
}

fn canot_change(some_string: &String) {
  // cannot borrow immutable borrowed content
  some_string.push_ptr(", world"); // ERROR!
}

fn change(some_string: &mut String) {
  some_string.push_ptr(", world");
}
```

- But mutable refernces have one big restriction: you can only have one mutable reference to a particular piece of data in particular scope. The benefit of having this restriction is that Rust can prevent data races at compile time. This code will fail:

```rust
let mut s = String:from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

- Multiple mutable refernes by creating a new scope

```rust
let mut s = String:from("hello");

{
  let r1 = &mut s;
} // r1 goes out of scope here, now we can make a new reference

let r2 = &mut s;
```

- Immutable references, Cannot have a mutable reference while we have an immutable one.

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s; // ERROR!
```

### Dangling References

- In Rust, the compiler guarantees that references will never be dangling reference

```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello"); // s is a new String in dangle

  &s // ERROR! missing lifetime specifier
} // s goes out of scope, and is dropped. It's memory goes away

fn no_dangle() -> String {
  let s = String::from("hello");

  s
}
```

## The Slice Type

- Slice doesn't have ownership
- Slice let you reference a contiguous sequence of elements in a colleaction rather than the whole collection

```rust
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  // use reference
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s);
  s.clear(); // empties the String, making it euqal to ""
}
```

### String Slices

- string slice is a reference to part of a String

```rust
let s = String::from("hello world");

// = not including `end`
let hello = &s[0..5];
let world = &s[6..11];

// = including end
let hello = &s[0..=4];
let world = &s[6..=10];

// with ...
let slice = &s[0..2];
let slice = &s[..2];

// drop the tralling number
let let = s.len();

let slice = &s[3..len];
let slice = &s[3..];

// both drop
let slice = &s[0..len];
let slice = &s[..];
```

- Attemping to creat a string slice in the middle of a multibyte character, it program will exit with an error
- Revision of `first_word`

```rust
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if (item == b' ') {
      return &s[0..i];
    }
  }

  &s[..]
}
```

- If we have an immutable reference to something, we `cannot` also take a mutable reference

```rust
fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s); // immutable borrow occurs here

  s.clear() // error, mutable borrow occurs here

  println!("the first word is: {}", word); // borrow later used here
}
```

- String literals are slices

```rust
let s = "Hello, world!"; // s is &str, slice pointing, immutable reference
```

- String Slices as Paramters

```rust
// We can pass string slice directly or
// slice of the entire String
fn first_word(s: &str) -> &str {
  ...
}

fn main() {
  let my_string = String::from("hello world");
  let word = first_word(&my_string[..]);

  let my_string_literal = "hello world";
  let word = first_word(&my_string_literal[..]);
  // string literals are string slice
  let word = first_word(my_string_literal);
}
```

- Other slices, this slice has the type `&[i32]`, works the same way as string slice do

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

# Using Structs to Structre Related Data

## Defining and Instantiation Structs

- The pieces of a struct can be different type, can be named

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}
```

- Create an instance by specifying concreat values

```rust
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true,
  sign_in_count: 1,
}
```

- Get value with dot notation. it the instance is mutable, we can change a value by using the dot notation
- Certain field as mutable is not allowed

```rust
let mut user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true,
  sign_in_count: 1,
}

user1.email = String::from("someone@example.com");
```

- Builder function

```rust
// implicity return that new instance

fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}

// shorthand version
fn build_user(email: String, username: String) -> User {
  email,
  username,
  active: true,
  sign_in_count: 1,
}
```

- Creating instances from other instnace with update value, or struct update syntax

```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("another"),
  active: user1.active,
  sign_in_count: user1.sign_in_count
};

// using struct update syntax `..`
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("another"),
  ..user1
}
```

- Tuple structs, can add meaning the struct name but not names associated with their field

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- Unit-like structs, don't have any fileds, behave simliarly to `()`

## An Example Program Using Struct

- More meaning

```rust
// with parameters
fn area(width: u32, height: u32) -> u32 {
  width * height
}

area(width, height);

// with tuple
fn area(dimensions: (u32, u32)) -> u32 {
  return dimensions.0 * dimensions.1
}

area((30, 50));

// with struct
struct Rectangle {
  width: u32,
  height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

let rect = Rectangle { width: 30, height: 50 };

area(&rect);

```

- Adding useful functionality with derived traits

```rust
[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect 1 is {:?}", rect1);

    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50
    // }
    println!("rect 1 is {:#?}", rect1);
}
```

## Method Syntax

- Methods are similar to functions, methods are different from functions in that they're defined within the context of a struct

### Defining Methods

- &self instead of rectangle: &Rectangle
- `&mut self`, to take an ownership

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  )
}
```

- `Automatic Referencing and Derefencing`, rust automatically adds in `&, &mut or *`

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

### Methods with More Parameters

```rust
impl Rectangle {
  fn area(&self) -> u32 {...}
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```

### Associated Functions

- Assoociated functions, without `self` parameter, still functions, not methods
- Often used for constructor that will return a new instance of the struct

```rust
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

let sq = Rectangle::square(3);
```

### Multiple impl Blocks

```rust
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```

# Enums and Pattern Matching

## Defining an Enum

````rust
enum IpAddrKind {
  V4,
  V6,
}

```rust
// assign
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// define a function
fn route(ip_type: IpAddrKind) {...}

struct IpAddr {
  kind: IpADdrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
}

// associated String
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::From("127.0.0.1"));
let loopback = IpAddr::V6(String::From("::1"));

// different types and amounts of associated data
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(String::From(127, 0, 0, 1));
let loopback = IpAddr::V6(String::From("::1"));

// embedded struct
struct Ipv4Addr {
  ...
}

struct Ipv6Addr {
  ...
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr)
}

// wide variety of type embedded
enum Message {
  Quit,
  Move { x:i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn call(&self) {
    ...
  }
}

let m = Message.Write(String::from("hello"));
m.call();
````

## The `Option` Enum and Its Advantages Over Null Values

- Another enum, compiler can check whether you've handled all the cases
- Option<T> is defined by the standard library
- Option value, `Some`, contains a value, or `None`, does not

```rust
enum Option<T> {
  Some(T),
  None,
}

// hold number type
let some_number = Some(5);

// hold string type
let some_string = Some("a string")

// hold none value
let absent_number: Option<i32> = None;

let x: i8 = 5;
let y: Options<i8> = Some(5);

let sum = x + y; // error, i8 + std::option:Option<i8>

// uee the value with various method, https://doc.rust-lang.org/std/option/enum.Option.html
let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));

let x = Some("value");
assert_eq!(x.expect("the world is ending"), "value");

let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);

let x: Option<u32> = None;
assert_eq!(x.is_some(), false);

let x: Option<u32> = Some(2);
assert_eq!(x.is_none(), false);

let x: Option<u32> = None;
assert_eq!(x.is_none(), true);
```

- Sample uses

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// The return value of the function is an option
let result = divide(2.0, 3.0);

// Pattern match to retrieve the value
match result {
    // The division was valid
    Some(x) => println!("Result: {}", x),
    // The division was invalid
    None    => println!("Cannot divide by 0"),
}
```

## The `match` Control Flow Operator

### Match with enum, returning function, bind to value

```rust
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    // match enum and return function
    Coin::Peny => {
      println!("Lucky penny!");
      1
    }
    // match enum and return value
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // patterns that bind to values, value_in_cents(Coin::Quarter(UsState::Alaska))
    Coin::Quater(state) => {
      println("State quater from{:?}!", state);
      25
    },
  }
}
```

# Matching with `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = plus_one(five); // returns 6
let none = plus_one(None); // No value to add to, stops and return `None`
```

### Matches are exhaustive

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1), // Error, didn't cover every possible case, `Pattern 'None` not covered
  }
}
```

### The `_` Placeholder

Use this when we don't want to list all possible values. `_` pattern will match any value.

```rust
let some_u8_value = 0u8;
match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  5 => println!("five"),
  7 => println!("seven"),
  _ => (), // By putting it after other arms, the `_` will match all the possible cases
}
```

## Concise Control FLow with `if let`

- `if let` syntax lets you combine `if` and `let` into a less verbos way to handle value that match one pattern while ignoring the rest

### Match one pattern

`if let`: syntax sugar for a match that runs code when the value matches one pattern

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
  Some(3) => println!("three"),
  _ => (),
}

// only work woth Some(3), using if let instead of

if let Some(3) = some_u8_value {
  println!("three");
}
```

### With else

```rust
fn main() {
  let mut count = 0;
  if let Coin::Quater(state) = coin {
    println!("State quater from {:?}!", state);
  } else {
    count + 1;
  }
}
```

# Packages, Crates, and Modules

- Packages are a Cargo feature that let you build, test, and share crates
- Crates are tree of modules that produce a library or executable
- Modules and the use keyword let you control the scope and privacy of paths
- A path is a way of naming an item such as a struct function, or module

## Packages and Crates for Making Libraries and Executables

- crate,
  - is a binary or library
  - crate root is a source file
  - know how to build a crate
- package,
  - has a Cargo.toml
  - describe how to build one or more crates

```rust
// package
.
├── Cargo.toml
├── bin // package can have multiple binary crates
└── src
    ├── lib.rs // crate root, it can be library or binary
    └── main.rs // crate root, it can be binary

```

## The Module System to Control Scope and Privacy

### Module

```rust
mod sound {
  mod instrument {
    fn guitar() {
    }
  }

  mod voice {}
}

fn main() {

}
```

### Path

```rust
// absolute path
crate::sound::instrument::guitar();

// relative path
::sound::instrument::guitar();
```

### Module as the Privacy Boundary

- Module are the privacy boundary in Rust
- All items (functions, methods, structs, enums, modules and constants) are private by default
- You can use `pub` keyword to make an item public
- You aren't allowed to use private code defined in modules in children of the current module
- You are allowed to use any code defined in ancestor modules, or the current module

### Using the `pub` Keyword to Make Items Publicet

- using in error

```rust
mod sound { // sound is allowed to use because it placed in front of instrument
  pub mod instrument {
    fn guitar() {}
  }
}

// absolute path. error, guitar is not allowed to use by pub
crate::sound::instrument::guitar();
```

- using in pub
```rust
mod sound {
  pub mod instrument {
    pub fn guitar() {}
  }
}

crate::sound::instrument::guitar();
```

### Starting Relative Paths with `super`

- the path starts from the parent module
- super is doing, is like starting a file system path with `...`
- in case of super is root

```rust
mod instrument {
  fn guitar() {
    // go to the parent module of instrument, the root
    super::breathe_in();
  }
}

fn breath_in() {}
```

- in case of super is mod

```rust
mod sound {
  mod instrument {
    fn guitar() {
      // go to the parent module of instrument, the root
      super::breathe_in();
    }
  }
  fn breath_in() {}
}
```

### Using `pub` with Structs and Enums

- struct

```rust
mod plant {
  pub struct Vegetable {
    pub name: String,
    id: i32
  }

  impl Vegetable {
    pub fn new(name: &str) -> Vegetable {
      Vegetable {
        name: String::from(name),
        id: 1,
      }
    }
  }
}

fn main() {
  let mut v = plant::Vegetable::new("squash");
  
  v.name = String::from("butternut squash");
  println!("{} are delicious", v.name);

  // error, id is private
  println!("The ID is {}", v.id);
}
```

- enum, all of its variants are public

```rust
mod menu {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

fn main() {
  let order1 = menu::Appetizer::Soup;
  let order2 = menu::Appetizer::Salad;
}
```

### The `use` Keyword to Bring Paths into a Scope

```rust
mod sound {
  pub mod instrucment {
    pub fn guitar() {}
  }
}

use crate::sound::instrument;

mod performance_group {
  use crate::sound::instrument;

  pub fn guitar_trio() {
    instrument::guitar();
    instrument::guitar();
    instrument::guitar();
  }
}

fn main() {
  instrument::guitar();
  instrument::guitar();
  instrument::guitar();

  performance_group::guitar_trio();
}
```

### Idiomatic `use` Paths for Functions vs. Other Items

```rust
mod sound {
  pub mod instrument {
    pub fn guitar() {}
  }
}

// specific the function's parent module with `use` not function name directly
use crate::sound::instrument::guitar;
// specific path item for structs, enums, and other items
use std::collections::HashMap;

fn main() {
  
  guitar();

  
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```

### Renaming Types Brought Into Scope with the `as` Keyword

- Two types the sanme name into the same scope

```rust
use std::fmt;
use std::io;

fn function1() => fmt::Result {}
fn function2() => io::Result<()> {}
```

- renaming

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() => Result {}
fn function2() => IoResult<()> {}
```

### Re-exporting Names with `pub use`

```rust
mod sound {
  pub mod instrument {
    pub fn guitar() {}
  }
}

mod performance_group {
  pub use create::sound::instrument;
}

fn main() {
  performance::group::instrument::guitar();
}
```

### Using External Packages

- Add `dependencies` with name and version of the package to `Cargo.toml`

```toml
[dependencies]
rand = "0.5.5"
```

- use

```rust
use rand::Rng;

fn main( ){
  let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

- for std, we don't need to add `dependencies` to `Cargo.toml`

```rust
use std::Hashmap
```

### Nested Paths for Cleaning Up large `use` Lists

```rust
use std::cmp::Ordering;
use std::io;

// using this instead of above

use std::{cmp::Ordering, io};
```

- Duplicate path by `self`

```rust
use std::io;
use std::io::Write;

// using this instead of above

use std::{self, Write}
```

### Bring All Public Definitions into Scope with the Glob Operator

```rust
use std::collections::*;
```

### Separating Modules into Different Files

- src/main.rs

```rust
mod sound;

fn main() {
  crate::sound::instrument::guitar()
  sound::instrument::guitar()
}
```

- src/sound.rs. `mod sound` load the content in `src/sound.rs`

```rust
pub mod instrument
```

- src/sound/instrument.rs

```rust
pub fn guitar() {}
```

# Common Collections

- vector: allows you to store a variable number of values next to each other
- string: collection of characters
- hash map: allows you to associate a value with a particular key

## Storing Lists of Values with Vectors

[samples]()

### Reading Elements of Vectors

- out of index: [] will cause the panic, `get` method returns `None` without panicking

- can' have mutable and immutable references in the same scope

```rust
let mut v = vec![1, 2, 3, 4];
let first = &v[0]; // immutable borrow
v.push(6); // mutable borrow, vector might require allocating new memory and copy the old elms
println!("fist is {}", first) // immutable borrow used here
```

### Iterating over the values in a Vector

[samples]()

## Using an enum to store multiple types

- vector can only store values that are the sample type
- store different type with enum

```rust
enum SpreadsheetCell {
  Int(32),
  Float(f64),
  Text(String)
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue"))
  SpreadsheetCell::Float(10.12)
]
```

## Storing UTF-8 Encoded Text With Strings

- String are implemented as a collection of bytes
- String, gowable, mutable, owned, UTF-8 encoded type
- string slice &str, borrowed
- OsString, OsStr, CString, Cstr

### Creating a New String

```rust
let mut s = String::new();
let data = "initial contents"
let s = data.to_string();
let s = "initial contents".to_string();
let s = String::from("initial content");
```

### Updading a String

```
let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.pus_str(s2); // string slice, no ownership about s1, foobar

//single character
s.push('l'); 

// ownership
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
// note s1 has been moved here and can no longer be used
// s1 called add(self, s:&str), add takes onwnership from self
// &s2 use deref coercion, doesn't takes ownership
let s3 = s1 + &s2; 

// format!
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

// indexing
// Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)).
let s1 = String::from
let h = s1[0]; // error cannot be indexed by integer

// len, String is a warpper over a `Vec<u8>
let let = String::From("hola").len(); // byte length, 4
let len = String::from("Здравствуйте").len(); // bytes length, 24
let hello = "Здравствуйте"; 
let answer = &hello[0]; // return not 3, 208, which first byte

// “नमस्ते” written in the Devanagari script is stored as a vector
// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

// Slicing strings
let hello = "Здравствуйте";
let s = &hello[0..4]; // Зд
let s = &hello[0..1]; // panic, index is not a char boundary

// Iterating
for c in "नमस्ते".chars() {
  println!("{}", c); // न म स ् त े 
}

for c in "नमस्ते".bytes() {
  println!("{}", c); // 224 164 // --snip-- 165 135
}

```

## Storing Keys with Assoicated Values in Hash Maps

- type `HashMap<K, V> stores a mapping of keys of type K to values of types V
- store data on the heap

```rust
// creating
use std::collection::Hashmap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// zip
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// HashMap<_, _>, rust infer the type
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// Ownership
let field_name = String::from("Favorite color");
let field_value = String::from("Blue":);

let mut map = HashMap::new();
map.insert(field_name, filed_value); // Type that copy trait, value are copied into the hash, no longer valid, fileds

// Accesing
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 20);

let team_name= String::from("Blue");
let score = scores.get(&team_name); // result Some(&10),result wrappper Some, get returns Option(&v), if there's no value will return None

// Iterating
for (key, value) in &scoress {
  println!("{}: {}", key, valye);

// Update a Hashmap

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // overwrite
scores.entry(String::from("Blue")).or_insert(50); // insert if the key has no value

// Update a value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1; // mutable reference
}
```

# Errror Handling

- Two types of categories:
  - recoverable: file not found, Result<T, E>
  - unrecoverable: symptoms of bugs, accessing end of an array, panic!

## Unrecoverable Erros With panic!

- call panic! when you stop the program, unwind and clean stack, and quit
- using 'abort' stop immediately

```
[profile.release]
panic = 'abort'
```

- using a panic! Backtrace `RUST_BACKTRACE=1 cargo run`
- cargo symbol are enabled by default `cargo build, cargo run` without `--release`

## Recoverable Error with Result

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- when ssucceeds return Ok(T), failed, return Err(E)

```rust
use std:fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening file: {:?}", error)
    },
  };
}
```

### Matching on Different Errors

- basics

```rust
use std:fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) = fc,
        Err(e) = panic!("Tried to create file but there was a problem {:?}", e),
      },
      other_error => panic!("There was a problem opening file: {:?}", error)
    },
  };
}
```

- closures

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File.open("hello.txt").map_err(|error| {
    if (error.kind()) == ErrorKind.NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Tried to create file but there was a problem {:?}", e)
      })
    } else {
      panic!("There was a problem opening file: {:?}", error)
    }
  })
}
```

### Shortcuts for Panic on Error: unwrap and expect

```rust
use std:fs:File;

fn main() {
  // unwarp, shortcut method of match, return Ok or Err
  let f = File::open("hello.txt").unwrap()

  // expect, let us choos the panic, return the fild handle or call panic! macro
  let f = File::open("hello.txt").expect("Failed to open hello.txt)
}
```

### Propagating Errors

```rust
fn read_username_from_file() -> Rsult<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  }

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Errr(e) =>> Err(e),
  }
}

// shrotcut for propagating?
fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?; // ? placed after a Result, work in same way as match, as getting error, will be return
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}


fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}

// ? operator can only be used in functions that return Result
fn main() {
  let f = File::open("hello.txt"); // error
}

fn main() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt"); // error

  Ok(())
}
```

## To panic! or Not ro panic!

default is Result but below cases are need to use panic!

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}

// panic! better, use invalid variable from out of context

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

# Generic Types, Traits, and Lifetimes

- Remove duplication in Rust

## Generic Data Types

- In Struct Definition

```rust
struct Point<T> {
  x: T,
  y: T,
}

let work = Point { x: 5, y: 6 }
let wont_work = Point { x: 5, y: 6.0 }

struct Point<T. U> {
  x: T,
  y: U
}

let both_integer = Point { x: 5, y: 20 }
let both_float = Point { x: 5.0, y: 20.0 }
let integer_and_float = Point { x: 5, y: 20.0 }
```

- In Enum Definition

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- In Method Definition

```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

//. implemention with f32
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

let p = Point{ x: 5, y: 10 };
println("p.x = {}", p.x())

// with another signatures
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);

printnl!("p3.x = {}, p3.y: {}", p3.x, p3.y)
```

### Performance of Code Using Generics

- Monomorphization is the process of turning generic ode into specific code by filling the concrete types that are used when compiled

## Trais: Defining Shared Behavior

- shared behavior in an abstract way
- interface, often called
- can't implement external traits on external type

```rust
// define
pub trait Summary {
  fn summerize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summerize(&self) -> String {
    format!("{}, by {} ({})", self.haedline, self.author, self.location)
  }
}

let news  = NewsArticle {
  username: String::from("username"),
  location: String::from("location"),
  author: String::from("author"),
  content: String::from("of course"),
}

println!("{}", news.summerize());

// default implementation, src/lib.rs
pub trait Summary {
  fn summerize_author(&self) -> String;
  fn summarize(&self) -> String {
    String::from("(Read more... form {})", self.summerize_author())
  }
}

impl Summary for Tweet {
  fn summerize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// traits as argumants, `impl Traits`
pub fn notify(item: impl Summary) {
  ...
}

// trait bound for more comples one, item, item2 must be same type
pub fn notify<T: Summary>(item: T, item2: T) {
  ...
}

// specify multiple traits with +
pub fn notify(item: impl Summary + Display) {
  ...
}

pub fn notify<T: impl Summary + Display>(item: T) {
  ...
}

// where clauses for clearer code
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: T) -> i32 {
  ...
}

// ->

fn some_function<T, U>(t:T, t:U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
  ...
}

// returning
fn returns_summarizable() -> impl Summary {
  ...
}

// compare stack-only data with copy
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {}

// conditionally implement methods, Pair<T> only implement cmd_display
// method if its inner type T implements the `PartialOrd`
impl<T: Display + PartialOrd> Pair<T> {
  ...
}
```

## Validating References with Lifetimes

- every reference in Rust has a lifetime

```rust
// preventing dangling references with lifetime
{
  let r // null
  {
    let x = 5;
    r = &x; // borrow
  } // x dropped
  println!("{}", r);
}

// borrow checker
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

// fixed
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}

// generic lifetimes in function
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let string1 = String::from("abcd");
let string2 = "xyz";

let result = longest(string1.as_str(), string2); // longest take a ownership
println!("The longest string is {}", result);

// life time annotation
// &i32, a refernce
// &'a i32, a refernce with an explicit lifetime
// &'a mut i32, a mutable reference with an explicit lifetime
// longest doens't need to know exacly how long args will live
// borrow checker shoud reject any values that adhere to these constraints
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  ...
}

let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str()); // borrow occurs, return string
} // string2 dropped here
println!("The longest string is {}", result);

// lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
  part: &'a str,
}

let novel = String::from("call me Ishmael. Some years ago...");
let first_sentence = novel.split('.')
  .next()
  .expect("Cound not find a '.'")
let i = ImportantExcerpt { part: first_sentence }; // can’t outlive the reference it holds in its part field.
```

# Testing
## Writings tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this tesdt fail")
    }

    #[test]
    fn boolean_test() {
        assert!(true)
    }

    fn panicer() {
        panic!("panic!")
    }

    #[test]
    #[should_panic()]
    fn should_panic() {
        panicer()
    }

    #[test]
    #[should_panic(expected ="panic!")]
    fn should_panic_expect() {
        panicer()
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Running tests

- tests in parallel or conscutively `$ cargo test -- --test-threads=1`
- output with `println`
- running test with the name of the test `cargo test test_name`
- ignore test with `#[ignore]` and `cargo test -- --ingore`

## Test Organinzation
- `#[cfg(test)]` annotation on the tests module tells Rust run test code only
- integration test with `tests` directory, without `#[cfg(test)]`
- using `setup` function at `tests/common.rs` to setup test
- can't test witn src/main.rs, binary project. use lib.rs instead of

# An I/O Project: Building a Command Line Program

## Accepting Command Line Arguments

- if any arguments contains invalid Unicode, panic will be raised
- first vector is the name of our binary
- `:?` debug formatter for vector
- guildline for task
  - Split your program into a main.rs and lib.rs, and move your program's logic to lib.rs
  - As long as your command line parsing logic is small, it can remain in main.rs
  - When the command line parsing logic starts getting complicated, extract it from main.rs adn move it to lib.rs
  - for main function
    - Calling the command line parsing logic with arguments values
    - Setting up any other confiuration
    - Calling a rin function in lib.rs
    - Handling the error if run return an error
- unwrap_or_else, defined on Result<T, E> by the standard library, allow us to define some custom non-panic! error handling
- `Box<dyn Error> is trait object to return Error trait
- `if let` and `unwrap_or_else` is same
- `'a` lifetime parameters specify, lifetime is connected to the lifetime of the return value. in this cases, the return vector that reference slices of the arguments contents
- `is_err` will return false, env variable is set to anything
- `eprintln!` macro that prints to the standard error stream, 

# Functional Language Features: Iterators and Closures


## Closures: Anonymous Functions that Can Capture Their Environment

- Rust's closures are anonymouse function you can save in a variable or pass as arguments to other function
- cakk the closures to evaluate it in a different context
- closures can capture values from the scope in which they're defined

```rust
let expensive_closure = |num| { // closure definition comes after the `=`, start with '|', had one more parameter
  println!("caclulating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
}
```

- with annoationg of the type

```rust
let expensive_closure = |num: u32| -> u32 {}
```

- more syntax

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

- types are locked in to the closure, getting a type error if we try to use a different type with the same closure

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello")); // String locked in 
let n = example_closure(5); // error
```
- captureing the environment with closures

```rust
fn main() {
  let x = 4;

  let equal_to_x = |z| z == x; // capure x

  let y = 4

  assert!(equal_to_x(y));

  let x = 4;

  // can't capture, use FnOnce, fnMut, Fn
  // FnOnce, capture from enclosing scope, take ownership, can't take ownership more than once
  // FnMut, can change the environment
  // Fn, borrows values from environment immutably
  fn euqal_to_x(z: i32) -> bool { z == x } 

  // move
  let x = vec![1, 2, 3];
  let equal_to_x = move |z| z == x; // x moved in to closure

  println!("can't use x here: {:?}", x); // panic! value used after move

  let y = vec![1, 2, 3];

  assert!(equal_to_x(y));
}
```

## Processing a Series of Items with Iterators

- In Rust, iterators are lazy, have no effect until you call methods that consume the iterator to use it up
- Iterator trait hs a number of different method with default implementation provide by the standard library

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
let sum v1_iter.sum();
```

- `iterator adaptors`, allow you to change iterators into different kinds of iterator, but all iterator are lazy, you have to call one of the consuming adaptor method to get result from calls iterator adaptors.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

- creating own iterator

```rust
struct Counter {
  count: u32;
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;

    if (self.count < 6) {
      Some(self.count)
    } else {
      None
    }
  }
}

let mut counter = Counter::new();

assert_eq!(counter.next(), Some(1));

let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                              .map(|(a, b)| a * b)
                              .filter(|x| x % 3 == 0)
                              .sum();
assert_eq!(18, sum);
```

## Improving Our I/O Project

- using iterator rather than vec!

```rust
impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let query = match args.next() {
      ...
    }

    let file.name = match args.next() {
      ...
    }

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
  }

  pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
  }
}

```

## Comparing Performance: Loops vs. Iterators

- iterator version is slightly faster
- Rust knows that how many iterations, so it `unrolls` the loop, compiles down to same assembly, `unrolling` is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

# More about Cargo and Crates.io

## Customizing Builds with Release Profiles

- build with profile name

```
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
```

- Cargo has default settings for each of the profiles that apply when there aren’t any [profile.*] sections in the project’s Cargo.toml file
- By adding [profile.*] sections for any profile you want to customize, you can override any subset of the default settings.

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publishing a Crate to Crates.io

- making useful documentation comments with `///`

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

- `cargo doc --open` will build the HTML
- other sections, `Panics`, `Erros`, `Safety`
- `cargo test` will run the code example in your documentation as tests!
- adding documentation comments with `//!`
- re-export with `pub use` statements the items at the top level

```rust
// src/lib.rs
pub use self::kinds::PrimaryColor;

// src/main.rs
use art::PrimaryColor;
```
- `cargo login` for setting up a crates.io account
- adding metadata

```
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

- `cargo publish` for publishing to crates.io
- `cargo yank --vers 1.0.1` for removing version from crates.ios

## Cargo Workspaces

- workspace, for multiple libary crates with `cargo new` and `[workspace]`

```
// Cargo.toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

- workspace directories

```
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

- dependencies to workspace

```
[dependencies]

add-one = { path = "../add-one" }
```

- worksapce has only one Cargo.lock at the top level of the workspace, which ensure that all crates are using same version of all dependencies
- `cargo test -o add-one` for testing specific test

## Installing Binaries from Crates.io with cargo install

- cargo install command allows you to install and use binary crates locally

## Extending Cargo with Custom Commands

- cargo is designed so you can extend it with new subcommands without having to modify Cargo. if a binary in your $PATH is named cargo-something, you can run it as if it was a cargo subcommand by running `cargo something`
- `cargo --list` for lising

# Smart Pointers

- Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities
- In Rust, which uses the concept of ownership and borrowing, an additional difference between references and smart pointers are pointers that only borrow data; in contrast, in many cases, `smart pointers own the data they point to`

## Using `Box<T> to Point to Data on he Heap

- Boxes allow you to store data on the heap
- Using a Box<T>

```rust

let b = Box::new(5); // b points the value 5, which is allocated on the heap
println!("{}", b);
// box goes out of scope, it will be deallocated
```

- Indirection data storing wiht box

```rust
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use crate::List::{Cons, Nil};

fn main() {
  let list = Cons(1, 
      Box::new(Cons(2,
        Box::new(Cons(3,
          Box::new(Nil)))));
}
```

## Treating Smart Pointers Like Regular References with the Deref Trait

- `Deref` trait allows you to customize the behavior of derefence operator *

```rust
let x = 5;
let y = &x;

assert_eq!(5, x);
assert_eq!(5, *y); // no implementation for `{integer} == &{integer}`

let y = Box::new(x);
assert_eq!(5, *y); // ok, Box poiting to the value x
```

- defining own smart pointer

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0 // returns a reference to the value we want access with *, * same as *(y.deref())
  }
}
```

- implic deref coercions with functions and methods

```rust
fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let m = MyBox::new(String::from("rust"));
  hello(&m); // reference to a MyBox<String>

  // if Rust didn't impkement deref coercion
  hello(&(*m)[..]);
}
```

- Rust does deref coercion when it finds types and trati implementations in three cases:
  - from `&T` to `&U` when `T: Deref<Target=U>`
  - from `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
  - from `&mut T` to `&U` when `T: Deref<Target=U>`

## Running Code on Cleanup with the `Drop` Trait

- drop to deallocate the space on the heap that the box points to

```rust
struct CustomSmartPointer {
  data: String;
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    ...
  }
}

fn main() {
  let c = CustomSmartPointer { data: String::from("my studff") };
  let d = CustomSmartPointer { data: String::from("my other studff") };

  // manually drop
  drop(d)
}


```

## Rc, the Reference Counted Smart Pointer

- to enable multiple ownership, Rust has a type called `RC<T>`

```rust
fn main() {
  let a = Rc::new(Cons(5, Rc:new(Cons(10, Rc::new(Nil)))));
  println!("count", Rc::strong_coint(&a)); // 1

  let b = Cons(3, Rc::clone(&a))
  println!("count", Rc::strong_coint(&a)); // 2

  {
    let c = Cons(3, Rc::clone(&a))
    println!("count", Rc::strong_coint(&a)); // 3
  }
  
  println!("count", Rc::strong_coint(&a)); // 4
}
```
## RefCell and the Interior Mutability Pattern

- RefCell<T> allows mutable borrows check at `runtime`. You can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable

```rust
sent_messages: RefCell<Vec<String>> = RefCell::new(vec![]);
send_messages.borrow_mut().push(String::from(message));
```

- Makeing two mutable refernces in same scope which isn't allowed

```rust
fn send(&self, message: &str) {
  let mut one_borrow = self.sent_messages.borrow_mut();
  let mut two_borrow = self.sent_messages.borrow_mut(); // already borrowed: BorrowMutError.

  one_borrow.push(String::from(message));
  two_borrow.push(String::from(message));
}
```

## Reference Cycles Can Leak Memory

- preventing memory leaks entirely is not one of Rust's guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust
- using Week<T>::upgrade to get Some or None whether Rc<T> has been dropped or not, and Rc::downgrade (to increase the weak_count), 

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leaf parent = None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // week node has child by downgrade
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
    // children: RefCell { value: [] } }] } })
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```


# Fearless Concurrency
## Threads
## Message Passing
## Shared State
## Extensible Concurrency: Sync and Send

# Object Oriented Programming Features of Rust
## Characteristics of Object-Oriented Languages
## Using Trait Objects that Allow for Values of Different Types
## Implementing an Object-Oriented Design Pattern

# Patterns Match the Structure of Values
## All the Places Patterns May be Used
## Refutability: Whether a Pattern Might Fail to Match
## All the Pattern Syntax

# Advanced Features
## Unsafe Rust
## Advanced Lifetimes
## Advanced Traits
## Advanced Types
## Advanced Functions & Closures
## Macros

# Final Project: Building a Multithreaded Web Server
## A Single Threaded Web Server
## Turning our Single Threaded Server into a Multithreaded Server
## Graceful Shutdown and Cleanup