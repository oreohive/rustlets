fn main() {
    use std::io::{stdin,stdout,Write};
    use std::vec::Vec;

    let mut choice=String::new();

    // Intro text
    println!(
"\nwelcome to the divisionator!
 \nenter a positive integer number; we'll find all positive integers it cleanly divides by.
 \nplease enter a positive integer: "
    );

    // flush buffer
    let _=stdout().flush();

    // take and read string, mutate var 'choice' to equal this string
    stdin().read_line(&mut choice).expect("oops! invalid string.");

    // remove \n (newline) characters
    if let Some('\n')=choice.chars().next_back() {
        choice.pop();
    }

    // remove \r (return) characters
    if let Some('\r')=choice.chars().next_back() {
        choice.pop();
    }

    // print choice
    println!("\nyou entered {}",choice);

    // make choice a 64-bit integer
    let choice_int: i64 = choice.parse().expect("failed to parse string to int");

    // print choice as int
    println!("your choice is: {}",choice_int);

    let mut successful = Vec::new();
    // if even, check all numbers
    if choice_int % 2 == 0 { 
        for n in 1..=choice_int {
            if choice_int % n == 0 {
                successful.push(n);
                }
            }
    }
    // if odd, only check odd numbers
    else {
        for n in (1..=choice_int).step_by(2) {
            if choice_int % n == 0 {
                successful.push(n);
                }
            }
    }

    println!("\n{} cleanly divides by {} positive integers:\n{:?}",choice_int,successful.len(),successful);
}