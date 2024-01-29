use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    string_on_stack();
}

fn word_guessing_game() {
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

fn string_on_stack() {
    // this string can be on the stack because it's size is known before compiling
    let mut s = "a string on the stack";
    print!("The string: {s}");

    s = "a string on the stack. edited";
    print!("The string edited: {s}")

    // as you can see you can still change the string, but you are not changing the string literal itself.
    // instead youre change which string s refers to. look at the example below for a string that you can change
}

fn string_in_heap() {
    let mut s = String::from("test");
    print!("The string: {s}");
    s.push_str(", another test");
    print!("The string: {s}");
}

fn copying_stuff_from_heap() {
    let s1 = String::from("hello");
    let s2 = s1;

    // using s1 here is impossible, since it has now been "moved" to s2. It is no longer valid as s1
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    println!("{}, world!", s2);
    // this makes sure that when both go out of scope, freeing up the memory is only called once (on s2).
    // and wont introduce any bugs
}

fn cloning() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    // cloning is an expensive operation.
    println!("s1: {s1}, s2: {s2}")
}

fn copying_stuff_on_stack() {
    let x = 5;
    let y = x;

    // using y here is POSSIBLE. Unlike the String type above. the size of x is known.
    // x lives entirely on the stack. which makes copying it very fast.
    println!("{}, world!", y);
}

fn copy_and_ownership_with_functions() {
    // copying and owning works the same when  passing vars to a function as it does with assigning to different vars
    let s = String::from("hello");
    takes_ownership(s);
    // using "s" leter in this scope is not possible, this would error.

    // you can hover make takes_ownership return the value after using it!
    let s = takes_ownership_and_returns(s);
    println!("{s}");

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn takes_ownership_and_returns(some_string: String) -> String {
    some_string
}

fn makes_copy(some_int: i32) {
    println!("{some_int}")
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing
// here we pass a reference to s1 to the calc_Len function. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
fn uses_reference() {
    let s1 = String::from("Hello");
    let len = calc_len(&s1);
    println!("Len of {s1} is: {len}")
}

fn calc_len(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn try_to_change_borrowed_var(s: &String) {
    // this does not work, bc this function does not own s
    // s.push_str(", world");
}

// to change a borrowed var, it needs to be mutable, and the function's signature also needs to accept a mutable reference
// you cannot lend out the mut s to two variables if both are used at the same time as in example shown below
fn change_mutable() {
    let mut s = String::from("hello");
    change(&mut s);

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}")
}

fn change(s: &mut String) {
    s.push_str(" world")
}

fn multiple_mutable_refs() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn mut_and_unmut_mixed() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// this does not work, because s is no longer available after this scope. making it a dangling reference.
// pointer pointing to nothing :))
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter() returns each element as a collection. enumerate() makes the result of each value in the collection a tuple of (index, item)
    // if we only used iter(), we would not know the index!
    for (i, &item) in bytes.iter().enumerate() {
        // b is a byte literal string
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_first_word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn string_slices() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
}

fn first_word_rewrite(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_literal_slices() {
    let my_string = String::from("Hello word");
    let hello = first_word_rewrite(&my_string[0..6]);
    let hello_world = first_word_rewrite(&my_string[..]);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_rewrite(&my_string);

    let my_string_lit = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let hello = first_word_rewrite(&my_string_lit[0..6]);
    let hello_world = first_word_rewrite(&my_string_lit[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let hello_world = first_word_rewrite(my_string_lit);
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn seed_users() {
    let user1 = build_user(String::from("bas1@gmail.com"), String::from("bas1"));

    let user2 = User {
        email: String::from("baswilson@gmail.com"),
        ..user1
    };

    // user1's fields that exist on the heap except for email have now moved to user2
    // we cannot access user1's username anymore
    println!("{}", user1.username)
}

// tuple structs
struct Color(i32, i32, i32);

fn tuple_stucts() {
    let black = Color(1, 1, 1);
}

// unit like structs
struct AlwaysEqual;

fn unit_like_structs() {
    let subject = AlwaysEqual;
    // honestly dont know the point of this. will probably become clear at some point
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// the implementation block of a struct can contain functions (methods)
// functions of a struct always have &self as first param. it borrows itself.
// the first param can be &self for shorter syntax. or self: &Self liek below
// can also take ownership if we want by just using &mut self: Self.
// here we only want to read, so we just borrow
// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax
impl Rect {
    fn area(self: &Self, multiplier: u32) -> u32 {
        self.width * self.height * multiplier
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(rect: Rect) -> u32 {
    rect.width * rect.height
}

fn calc() {
    let rect = Rect {
        width: 20,
        height: 20,
    };

    println!("rect 1 is {:?}", rect);
    // will output rect 1 is { width: 20, height: 20 }
    println!("rect 1 is {:#?}", rect);
    // will output rect 1 is {
    //    width: 20,
    //    height: 20
    // }
    // best to only use whend ebugging. remove when compiling for prod
    dbg!(&rect);

    println!("Area of rect is {}", rect.area(1));

    // we call this differently. it is kind of like a constructor function
    // rust automatically namespaced square under rect. you cannot call it like: Rect.square()
    let square = Rect::square(10);
}

enum IpAddrKind {
    V4,
    V6,
}

fn use_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4)
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_addr_enum_with_value() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// enum with all kinds of types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// can also add methods to a enum!?!?!
impl Message {
    fn call(&self) {}
}

fn use_crazy_enum() {
    let m = Message::Write(String::from("Red"));
    let q = Message::Quit;
    let mv = Message::Move { x: 10, y: 10 };
    let c = Message::ChangeColor(1, 1, 1);
    c.call();
    q.call();
    //etc...
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
fn use_option_enum() {
    let n = Option::Some(5);
    let s = Option::Some("char");
    let absent_number: Option<i32> = Option::None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // we can get the value of y. it might be nothing since its an option.
    let real_y = match y {
        Option::None => 0,
        Option::Some(y) => y,
    };

    let sum = x + real_y;
}

#[derive(Debug)] // so we can inspect the state in a minute
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn use_vinc() {
    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn use_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn catch_all_match() {
    let roll = 10;
    match roll {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        roll => move_player(roll),
    }
}

fn _match() {
    let roll = 10;
    match roll {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn _match_do_nothing() {
    let roll = 10;
    match roll {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        _ => (), // return empty tuple
    }
}

fn if_let() {
    // can be done with match
    let config_max = Option::Some(3u8);
    match config_max {
        Option::Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // but also with if let
    // max binds to the value inside the Some config_max. It is assigned with '='
    if let Option::Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // both do the same thing
}

fn if_let_else() {
    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // same as the _ catchall thing
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
