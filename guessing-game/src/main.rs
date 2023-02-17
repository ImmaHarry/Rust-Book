use rand::Rng; // Importing the random number gen library
use std::cmp::Ordering; // Importing the comparing library
use std::io; // Importing the input/output library

fn main() {
    println!("Welcome to the guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=100); //Gemerating a number [1,100]

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); //Creating a mutable string input that has no value

        io::stdin()
            .read_line(&mut guess)
            .expect("Program failed to read the line"); //Overwriting the value of "guess" to the new input

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //Making sure the guessed number is a "u32" for comparing, removing any white spaces/new lines with (trim) & converting to u32 with parse

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
