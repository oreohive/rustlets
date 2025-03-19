fn main() {
    use std::io::{stdin,stdout,Write};

    let mut farmer=0;
    let mut fox=0;
    let mut chicken=0;
    let mut grain=0;

    let mut choice=String::new();

    // Intro text
    println!(
"\nA fox, chicken && a bag of grain wait by the side of a river.\n
 \nWhich item will you take in your rowboat to the other side?\n
 \nPlease input 'fox', 'chicken', 'grain' or 'farmer':\n"
    );

    // Flush buffer
    let _=stdout().flush();

    // Take and read string, mutate var 'choice' to equal this string
    stdin().read_line(&mut choice).expect("Oops! Invalid string.");

    // Remove \n (newline) characters
    if let Some('\n')=choice.chars().next_back() {
        choice.pop();
    }

    // Remove \r (return) characters
    if let Some('\r')=choice.chars().next_back() {
        choice.pop();
    }

    // Print choice
    println!("\nYour choice was: {}",choice);

    if choice == "farmer" && farmer == 0 {
        farmer = 1;};
    if choice == "fox" && farmer == 0 && fox == 0 {
        farmer = 1;
        fox = 1;};
    if choice == "chicken" && farmer == 0 && chicken == 0 {
        farmer = 1;
        chicken = 1;};
    if choice == "grain" && farmer == 0 && grain == 0 {
        farmer = 1;
        grain = 1;};
    
    output(farmer, fox, chicken, grain);

    if game_is_over(fox, chicken, grain) {
        println!("You lose.")}
    else {
        println!("You win!")}
}

fn output(farmer: i32, fox :i32, chicken: i32, grain: i32) {
    println!("\nSide 1:");
    if farmer == 0 {
        println!("Farmer")}
    if fox == 0 {
        println!("Fox")}
    if chicken == 0 {
        println!("Chicken")}
    if grain == 0 {
        println!("Grain")}
    println!("\nSide 2:");
    if farmer == 1 {
        println!("Farmer")}
    if fox == 1 {
        println!("Fox")}
    if chicken == 1 {
        println!("Chicken")}
    if grain == 1 {
        println!("Grain")}
}

fn game_is_over(fox :i32, chicken: i32, grain: i32) -> bool {
    if chicken == grain {
        println!("The chicken ate the grain.");
        return true;}
    if fox == chicken {
        println!("The fox tried to eat the chicken.");
        return true;}
    return false
}