use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let number_to_guess = rand::rng().random_range(1..=100);

    println!("The magic number is {number_to_guess}");

    loop {
        print!("Insert guess: ");
        io::stdout().flush().unwrap();

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"), 
            Ordering::Equal => {println!("Thats the number!"); break}
        }
    }
    
}
