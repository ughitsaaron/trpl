use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("Guess a number between {} and {}", MIN, MAX);

    // The upper bounds of `gen_range` is exclusive so we add one to the second
    // argument below
    let secret_number = rand::thread_rng().gen_range(MIN, MAX + 1);
    let mut num_guesses = 0;

    loop {
        println!("Please input your guess!");
        println!("You've guessed {} times", num_guesses);
        num_guesses += 1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{} is too large!", guess),
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Equal => {
                println!("The secret number is {}! You win.", guess);
                break;
            }
        }
    }
}
