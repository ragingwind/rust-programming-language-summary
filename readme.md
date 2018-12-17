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

###  Scalar Types

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
```
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
```
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
```
fn another_function(x) {
  println!("The value of x is: {}", x);
}

fn another_function(y: i32) {
  println!("The value of y is: {}", y);
}
```
- Statements and Expressions
```
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
```
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