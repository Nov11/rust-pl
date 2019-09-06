use std::io;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_tng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    println!("Guess the number");

    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("Your guess : {}", guess);
}
