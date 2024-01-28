use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");

        let mut guess: String = String::new();

        // expect returns the return value of the function called before it
        // only if the instance of Result is an Ok value
        let bytes: usize = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse returns an enum Ok and Err
        // so if we match with Ok we return the parsed uint
        // otherwise we return empty but print a line saying we should enter a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}, amount of bytes in anwser: {bytes}");

        // cmp returns an Ordening. It is the result of a comparison between two values.
        // we use match to show right println
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
            }
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
            Ordering::Greater => {
                println!("Too big");
            }
        }
    }
}
