#![allow(unused)]

/** Same as
 * use std::cmp::Ordering
 * use std::io */
use rand::Rng;
use std::{cmp::Ordering, io};

const MAX_ROUNDS: i32 = 5;

fn main() {
    // generate random number
    let secret = rand::thread_rng().gen_range(1..91);
    let mut rounds = 5;

    loop {
        if (rounds <= 0) {
            println!("The secret value was: {}", &secret);
            break;
        }

        println!("Rounds remaining {}/{}", &rounds, &MAX_ROUNDS);
        // Ask user a number
        println!("Please enter a number: ");
        // Read user's input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // panik if error

        // Verify user's number and tell

        /** Shadowing previous variable */
        let guess: i32 = guess.trim().parse().expect("Please type a number");
        println!("Your guess is: {}", &guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small.\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("You won! \n");
                break;
            }
        }
        rounds -= 1;
    }
    println!("Thanks por playing!!\n")
}
