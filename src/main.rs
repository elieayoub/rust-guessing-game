// equivalent to use rand
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //It’s inclusive on the lower bound, but exclusive on the upper bound, so we need 1 and 101 to get a number ranging from one to a hundred
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // by default let is immutable. So we set 'mut' next to it to make it mutable
        let mut guess = String::new();

        // If we didn’t use std::io, we could have written this line as std::io::stdin()...
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //handling exception with a message

        //convert string to unsigned integer 32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // we just use the catch all _ instead of a name. This catches everything that isn't Ok
            Err(_) => continue,
        };

        //The {}s are a placeholder, and so we pass it guess as an argument. If we had multiple {}s, we would pass multiple arguments
        //let x = 5;let y = 10;println!("x and y: {} and {}", x, y);
        println!("You guessed: {}", guess);

        //The cmp() method can be called on anything that can be compared
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                //quit when you win the game
                break;
            }
        }
    }
}