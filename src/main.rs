use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome, this is the best number guessing game on the block!");

    loop {
        println!("Guess the number! (1-100)");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess!");

        loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read your guess");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number");
                    continue;
                }
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small, try again..."),
                Ordering::Greater => println!("Too big, try again..."),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

        let mut player_answer = String::new();

        println!("Would you like to play again? (y/n)");

        let mut close_game = false;
        loop {
            io::stdin()
                .read_line(&mut player_answer)
                .expect("Failed to read your answer");

            match player_answer.as_str().trim() {
                "y" => {
                    println!("Great, thanks for sticking around, hope you have fun!");
                    break;
                }
                "n" => {
                    println!("Thanks for playing, hope to see you again!");
                    close_game = true;
                    break;
                }
                _ => {
                    println!("Invalid option, please input y or n");
                    continue;
                }
            }
        }

        if close_game {
            break;
        }
    }
}
