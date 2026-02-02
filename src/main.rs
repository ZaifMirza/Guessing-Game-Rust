use rand::Rng;
use std::io;
use std::process::Command;

enum Guess {
    Correct,
    ToomuchHigh,
    TooHigh,
    BitHigh,
    ToomuchLow,
    TooLow,
    BitLow,
    Veryclose,
}

fn clear_screen() {
    // Works on Linux/macOS
    Command::new("clear").status().unwrap();
    // For Windows, uncomment this:
    // Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
}

fn main() {
    println!("========================================");
    println!("   WELCOME TO THE GUESSING GAME   ");
    println!("========================================");
    println!("Rules:");
    println!("1. Guess a number between 1 and 500.");
    println!("2. You have 10 attempts.");
    println!("3. Hints will guide you if you are high or low.");
    println!("4. If you get 'Very Close', you are within ±10.");
    println!("========================================\n");

    // Player 1 sets the secret number
    let mut input = String::new();
    println!("Player 1, please enter the secret number (1–500):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let secret_number: u32 = match input.trim().parse() {
        Ok(num) if num >= 1 && num <= 500 => num,
        _ => {
            println!("Invalid number. Using random number instead.");
            rand::thread_rng().gen_range(1..=500)
        }
    };

    // Clear screen so Player 2 can’t see
    clear_screen();

    let mut total_guess = 10;
    let mut win = false;

    while total_guess > 0 {
        println!("Please Input Your Number Gentleman:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 500 {
            println!("Give valid input between 1 to 500");
            continue;
        }

        let gap = if guess > secret_number {
            guess - secret_number
        } else {
            secret_number - guess
        };

        let result = if guess == secret_number {
            Guess::Correct
        } else if guess > secret_number {
            if gap >= 250 {
                Guess::ToomuchHigh
            } else if gap >= 125 {
                Guess::TooHigh
            } else if gap >= 10 {
                Guess::BitHigh
            } else {
                Guess::Veryclose
            }
        } else {
            if gap >= 250 {
                Guess::ToomuchLow
            } else if gap >= 125 {
                Guess::TooLow 
            } else if gap >= 10 {
                Guess::BitLow
            } else {
                Guess::Veryclose
            }
        };

        match result {
            Guess::Correct => {
                println!("You won!! Congratulations");
                win = true;
                break;
            }
            Guess::ToomuchHigh => {
                println!("[Hint] Kuch jyada upar pahoch gaye niche jump karo");
            }
            Guess::TooHigh => {
                println!("[Hint] High hai thoda niche aao");
            }
            Guess::BitHigh => {
                println!("[Hint] Bas thoda sa high hai niche aao");
            }
            Guess::ToomuchLow => {
                println!("[Hint] Kuch jyada niche gir gaye upar aao");
            }
            Guess::TooLow => {
                println!("[Hint] Bahot niche gir gaye upar aao thoda");
            }
            Guess::BitLow => {
                println!("[Hint] Niche ho thoda sa upar aao");
            }
            Guess::Veryclose => {
                println!("[Hint] You are very close to the number");
            }
        }

        total_guess -= 1;
        println!("Your total guesses left: {total_guess}\n");
    }

    if !win {
        println!("Game Over! The lucky number was: {}", secret_number);
    }
}
