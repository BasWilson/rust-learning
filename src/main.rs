use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let secret_number: u32 = get_random_number(1, 100);
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

fn tuples() {
    // a tuple!
    let x: (i32, &str, bool) = (500, "Test", false);

    // can be accessed in a couple of ways
    let fivehundred = x.0;
    let test = x.1;
    let false_haha = x.2;

    println!("{fivehundred} {test} {false_haha}");

    // or like so (destructring)
    let (fivehundred, test, false_haha) = x;

    println!("{fivehundred} {test} {false_haha}");
}

fn arrays() {
    // arrays have a fixed size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // you can also create an array like so, it will then hold that first index element x amount of times.
    // 3,3,3,3,3,3,3,3,3,3,3
    let b = [3; 10];

    let print_a = a[1];
    let print_b = b[2];

    println!("Accesing element {print_a} {print_b}")
}

fn print_function(x: String) {
    println!("{x}")
}

fn get_random_number(start: u32, end: u32) -> u32 {
    // because there is no semicolon, it will just return te result of this call
    rand::thread_rng().gen_range(start..=end)
    // below is the same function but more verbose
}

fn get_random_number_verbose(start: u32, end: u32) -> u32 {
    let n = rand::thread_rng().gen_range(start..=end);
    // returns the same thing
    return n;
}

fn get_random_number_more_verbose(start: u32, end: u32) -> u32 {
    let n = rand::thread_rng().gen_range(start..=end);
    // returns the same thing
    n
}

fn greater_than(x: u32, y: u32) -> bool {
    // see how there is the shorthand return again :) very cool
    if x > y {
        false
    } else {
        true
    }
}

fn greater_than_short(x: u32, y: u32) -> bool {
    x > y
}

fn comparison_as_let() {
    let money_made = if greater_than(1, 2) { 100 } else { 0 };
    println!("You made ${money_made}");
}

fn loops() {
    loop {
        // infinite untill break;
    }

    let mut counter = 0;

    // you can even return a value from a loop!!!
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
}

// you can name loops, and then break a nested loop by calling it's name;
// crazy shit
fn nested_loops() {
    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn while_loop() {
    let mut number = 3;

    // pretty straight forward
    while number != 0 {
        number -= 1;
    }
}

fn for_loop() {
    let a = [10; 5];

    for element in a {
        println!("e: {element}")
    }
}

fn for_rev() {
    let a = 1..4;
    for number in a.rev() {
        println!("{number}!")
    }
    println!("Lift off!!")
}
