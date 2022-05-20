use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!(
        "You start with 15 points. Try to guess the correct number before running out of points."
    );

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let default_score = 15;
    let mut score = &default_score;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInput must be a number.\n");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You kept {} points");
                break;
            }
        }
        println!("You lost a point.");
        score = score - 1;
        println!("You now have {} points left.", score);
    }
}
