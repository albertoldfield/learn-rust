use std::io;

fn main() {
    println!("Pick a number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline!");

    println!("You guessed {guess}");
}
