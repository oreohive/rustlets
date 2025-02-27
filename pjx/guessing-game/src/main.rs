/* pjx/guessing-game/src/main.rs */

use std::io; /* use standard io */

fn main() {
    println!("guess the number!");

    println!("please input your guess.");
    let mut guess = String::new(); /* mut makes this mutible */

    io::stdin().read_line(&mut guess)
        .expect("failed to read line!");

    println!("you guessed: {}", guess);
}
