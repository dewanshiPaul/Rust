use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("You are playing a game to guess the number");

    let gen_num: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess.[1 to 100]");

        let mut guess_num = String::new();

        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read");

        let guess: u32 = guess_num.trim().parse().expect("Please type a number");

        println!("You guessed: {guess_num}");

        match guess.cmp(&gen_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You guessed it correct");
                break;
            },
            Ordering::Greater => println!("Too big")
        }   
    }
}
