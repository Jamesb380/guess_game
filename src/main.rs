
extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("The guessing game");
    let secrect_number = rand::thread_rng().gen_range(1..101);

    loop{

    
        println!("Please enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
                             };
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secrect_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win");
                break;
            }
        }
    }
}
