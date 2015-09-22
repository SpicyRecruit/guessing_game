use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io.std().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");

    // Test //

    println!("You guessed: {}", guess);
}
