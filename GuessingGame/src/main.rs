use rand::Rng;
use std::io::{self, Write};

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess:i32 = get_intpu(String::from("Please enter your guess: "));

        if guess == secret_number {
            println!("You guessed the secret number.");
            break
        }
        else if guess < secret_number {
            println!("Higher!")
        }
        else if guess > secret_number {
            println!("Lower!")
        }
    }

}

fn get_intpu(text: String) -> i32{
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().parse::<i32>().unwrap()
}

mod tests {
    // use super::*;
}