use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    println!("Guess the number my master");

    let actual_number = rand::thread_rng().gen_range(1..=10);

    loop {
        print!("Input your guess: ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess master");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&actual_number){

            Ordering::Less => println!("You guessed {guess} it's too small\n"),
            Ordering::Equal => {println!("You guessed {guess} thats it\n"); break;},
            Ordering::Greater => println!("You guessed {guess} it's too big\n"),
        }
    }

}