use std::io;

fn add() {
    println!("Enter the first number");
    let mut first_value = String::new();
    let mut second_value = String::new();

    io::stdin()
        .read_line(&mut first_value)
        .expect("Couldn't get the first_value");

    println!("Enter the second number");

    io::stdin()
        .read_line(&mut second_value)
        .expect("Couldn't get the second_value");

    let first_value = first_value
        .trim()
        .parse::<u32>()
        .expect("Couldn't collect value");
    let second_value = second_value
        .trim()
        .parse::<u32>()
        .expect("Couldn't collect value");

    let result = first_value + second_value;
    println!(
        "The sum of {} and {} is: {}",
        first_value, second_value, result
    );
}

fn main() {
    add()
}
