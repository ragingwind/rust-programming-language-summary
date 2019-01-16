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
