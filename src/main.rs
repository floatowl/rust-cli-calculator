mod functions;
use std::io;

use crate::functions::take_input;
fn main() {
    functions::print_menu();

    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice);
    let choice: i32 = choice.trim().parse().expect("Failed to parse");

    match choice {
        1 => {
            println!("Please enter two numbers");
            let number1: f64 = take_input();
            let number2: f64 = take_input();
            let result: f64 = functions::add(number1, number2);
            println!("The sum is {}", result);
        }

        2 => {
            println!("Please enter two numbers");
            let number1: f64 = take_input();
            let number2: f64 = take_input();
            let result: f64 = functions::sub(number1, number2);
            println!("The difference is {}", result);
        }

        3 => {
            println!("Exitting the program");
            std::process::exit(0);
        }

        _ => {
            println!("Please enter a valid choice");
        }
    }
}