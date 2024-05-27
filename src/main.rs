use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome, this is the best number guessing game on the block!");

    run_guessing_game()
}

fn run_guessing_game() {
    loop {
        println!("Guess the number! (1-100)");

        play_game();

        if !check_if_player_wants_to_continue_playing() {
            return;
        }
    }
}

fn play_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess!");

    let mut guess = String::new();

    loop {
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
            Ordering::Less => {
                println!("Too small, try again...");
                continue;
            }
            Ordering::Greater => {
                println!("Too big, try again...");
                continue;
            }
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}

fn check_if_player_wants_to_continue_playing() -> bool {
    let mut player_answer = String::new();

    println!("Would you like to play again? (y/n)");

    loop {
        io::stdin()
            .read_line(&mut player_answer)
            .expect("Failed to read your answer");

        match player_answer.as_str().trim() {
            "y" => {
                println!("Great, thanks for sticking around, hope you have fun!");
                return true;
            }
            "n" => {
                println!("Thanks for playing, hope to see you again!");
                return false;
            }
            _ => {
                println!("Invalid option, please input y or n");
                continue;
            }
        }
    }
}
