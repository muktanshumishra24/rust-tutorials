use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Your Secret Number is: {}", secret_number);

    loop {
        println!("Please Input your Guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","You Win".green());
                break;
            }
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Less => println!("{}","Too Small".red()),
        }
    }
}
