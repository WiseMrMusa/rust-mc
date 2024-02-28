use std::{io, cmp::Ordering};
use rand::Rng;

pub fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess between 1 and 100");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Input a number between 1 and 100");
                continue
            },
        };
        
        // Compare the guess
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("Congratulations");
                break;
            }
            Ordering::Greater => println!("Your guess is greater than the secret number"),
            Ordering::Less => println!("Your guess is lesser than the secret number")
        }
    }

    
    // println!("You guessed: {}", guess);
    // println!("The secret number is {}", secret_number);

    // let x = 5;
    // let y = 7;
    // println!("m = {x}, q = {y}");
}
