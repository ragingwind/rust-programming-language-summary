use std::io;
use std::cmp::Ordering;
// use external crate rand, see Cargo.toml
use rand::Rng;

fn main() {
    // print message
    println!("Guess the number!");

    // gen_range is de
    let secret_number = rand::thread_rng().gen_range(1..=101);

    // {} is placeholder for variable
    println!("the serect number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // immutable variable to save user input
        let mut guess = String::new();

        io::stdin()
            // read stdio buffer and write it to guess
            .read_line(&mut guess)
            .expect("Failed to read line");

        // handling invalid input
        let guess: u32 = match guess.trim().parse() {
            // convert to u32
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match expression to compare guess and secret_number
        match guess.cmp(&secret_number) {
            // Ordering type is another enum, Less, Greater, Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // quite the loop
                break;
            }
        }
    }
}
