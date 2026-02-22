/* 
Brings the input/output library into scope.
By default, Rust has a set of items defined in 
the standard library that it brings into the scope 
of every program. This set is called the prelude, 
and you can see everything in it in the standard 
library documentation.
*/
use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() { // entry point into the program
    println!("Guess the number!"); // macro that prints a str to the screen

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // This var is not mutable.
    // thread_rng() allows us to use a rng that is local to the current thread 
    // of execution and is seeded by the OS.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // stores the user input as a var, new instance of a String type
        // rust variables are not mutable by default, so use "mut" keyword to make it mutable.
        // The :: syntax indicates that new() is an associated function of the String type.

        io::stdin()
        // The stdin function returns an instance of std::io::Stdin, 
        // which is a type that represents a handle to the standard input 
        // for your terminal.
        // Returns an value Result, an enum.
        // An enum is a type that can be one of multiple possible states.
        // We call each possible state a variant.
        // Result has two states: Ok and Err.
            .read_line(&mut guess)
            // Tells it to append the user input to guess.
            // References like these are immutable by default.
            // Need to make them mutable.
            .expect("Failed to read lined");
            // Handles potential failure.
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Shadows the previous value of guess with a new one, ie reuses the guess var.
        // Useful if you want to convert one type into another type as seen here.
        // parse() converts string to another type. It returns a Result.
        // u32 = unsigned 32-bit integer.
        // match here is doing error handling - checking if the input matches a num or not 

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // See explanation of 'match' on this page:
        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    }
}