use std::io;
use rand::prelude::*;

fn main() {
    println!("Welcome, Guessing Game!");

    let random_number = rand::rng().random_range(1..101);
    
    'game_loop: loop {
        let mut guess = String::new();
        println!("Enter a guess");
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read guess input");

        let number = guess
            .trim()
            .parse::<u32>()
            .expect("Unable to parse guess input");

        if number < random_number {
            println!("Too small!");
        } else if number > random_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break 'game_loop;
        }
    }
}