use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    let min = 1;
    let max = 100;
    let range = min..=max;
    println!("Guess the number from {min} to {max}!");

    let number = rand::thread_rng().gen_range(range);

    println!("Enter the guess.");
    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Equal => {
                println!("Gotcha!");
                break;
            }
            Ordering::Greater => println!("Number is too big"),
        }
    }
}
