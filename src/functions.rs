use std::io;

pub fn print_menu() {
    println!("Choose your operation!");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Exit");
}

pub fn add(number1: f64, number2: f64) -> f64 {
    return number1 + number2;
}

pub fn sub(number1: f64, number2: f64) -> f64 {
    return number1 - number2;
}

pub fn take_input() -> f64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    let input: f64 = input.trim().parse().expect("Failed to parse");
    return input;
}
