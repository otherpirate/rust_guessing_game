extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number", guess.trim());
                continue;
            },
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => msg_less(),
            Ordering::Greater => msg_great(),
            Ordering::Equal => {
                msg_win();
                break;
            }
        }
    }
}

fn msg_less() {
    println!("Too small");
}

fn msg_great() {
    println!("Too big");
}

fn msg_win() {
    println!("You win!");
}