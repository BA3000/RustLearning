use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // [1, 101), get random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the previously defined variable "guess"
        // trim() for eliminating spaces in the beginning and end, also removes \n, \r, or \r\n etc.
        // parse() is for parsing a string into some kind of number. target type is required, you can
        // specify type by using var_name: type_name like below.
        // expect() for catching errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid num!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        // values to be compared must have the same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
