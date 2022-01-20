use rand::Rng;
use std::io;
use colored::*;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess a number between 1 and 100");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok (num ) => num,
            Err (_) => {
                println!("number!");
                continue;
            },
        };
        if guess == secret_number {
            break;
        } else if guess > secret_number {
            println!("{}", "Too high".red());
        } else {
            println!("{}","Too low".red());
        }
        println!("{}","Guess again".yellow());
    }
    println!("{}","you won!".green())
}
