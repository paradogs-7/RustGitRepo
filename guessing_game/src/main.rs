use std::io;

fn main(){

    println!("Guess the number my master\n");

    println!("Input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read your guess master");

    println!("You guessed {guess}");
}