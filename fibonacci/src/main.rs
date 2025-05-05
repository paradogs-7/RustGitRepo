use std::io::{self, Write};

fn main() {

    println!("");

    let mut input = String::new();

    print!("Enter a number to calculate its Fibonacci sequence: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    
    let mut previous=1;
    let mut next=1;
    let mut holder=next;
    for _num in 1..n{
        next +=previous;
        previous=holder;
        holder = next;

    }
    println!("The {}th fibonacci number is {next}", n);
    println!("");

}