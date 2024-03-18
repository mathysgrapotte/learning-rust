use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range is a method of the Rng trait, which is implemented for the thread_rng function. The range is inclusive on both ends (1 and 100 are both possible values)

    println!("The secret number is: {secret_number}");

    println!("Please input your guess."); // println! is a macro that prints a string to the screen

    let mut guess = String::new(); // let creates a new variable, mut makes it mutable, String::new() creates a new instance of a string (essentially new is the constructor for the String type)

    io::stdin()
        .read_line(&mut guess) // read_line takes the user input and appends it to the string guess, we add a & to indicate that this argument is a reference (meaning a pointer to the value in memory)
        .expect("Failed to read line"); // read_line returns a Result type, which is an enum that has the variants Ok and Err. If the Result is an Err, expect will cause the program to crash and display the message passed to it

    println!("You guessed: {}", guess); // {} is a placeholder for the value of the variable passed to println! after the string
}
