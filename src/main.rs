use std::cmp::Ordering;
use std::io;

use rand::Rng;

//include a trait call Rng which has a method named gen_range

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

//    println!("The secret number is {}", secret_number);
    loop {
        println!("Guess the number");

        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line");

        println!("Your guess : {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
//            .expect("not a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
