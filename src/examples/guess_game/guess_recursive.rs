use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Try to guess the secret word ([1-100])...");
    let secret = rand::thread_rng().gen_range(1..=100);
    play(secret)
}

fn play(secret: i32) {
    let guesses: i32 = playGame(secret, 0);
    println!("{}", guesses)
}

fn playGame(secret: i32, try_count: i32) -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    match input.cmp(&secret) {
        Ordering::Less => {
            println!("Too small!");
            playGame(secret, try_count + 1)
        }
        Ordering::Greater => {
            println!("Too big!");
            playGame(secret, try_count + 1)
        }
        Ordering::Equal => {
            println!("You win!");
            try_count + 1
        }
    }
}
