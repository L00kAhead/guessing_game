use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("THIS IS A CLI GUESSING GAME");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");
        let mut guess: String = String::new();
        //::new() -> associated function that returns an empty string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read Line ");
        /* The & indicates that this argument is a reference, which gives 
        you a way to let multiple parts of your code access one piece of 
        data without needing to copy that data into memory multiple times.
        Like variables, references are immutable by default. Hence, you 
        need to write &mut guess rather than &guess to make it mutable.*/

        //Shadowing: redeclare the variable| changing from one type to another
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
