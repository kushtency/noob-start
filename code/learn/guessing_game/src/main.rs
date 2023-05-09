use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let num: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number !!!");

    println!("Please input your guess : ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("enter a number only");

        println!("you guessed: {guess}");
        match guess.cmp(&num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
