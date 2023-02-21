use std::io::{stdin};

// A simple calculator app that takes two numbers and an operator from a user and returns calculated result

fn main() {
    let mut first_number = String::new();
    println!("Please enter the first value");
    stdin().read_line(&mut first_number).expect("Failed to read first value");
    let first_number: f32 = first_number
        .trim()
        .parse()
        .expect("First value entered was not a number");

    let mut operator = String::new();
    println!("Please enter the operator (+, -, /, * or x or X)");
    stdin().read_line(&mut operator).expect("Failed to read operator");

    let mut second_number = String::new();
    println!("Please enter the second value");
    stdin().read_line(&mut second_number).expect("Failed to read second value");
    let second_number: f32 = second_number
        .trim()
        .parse()
        .expect("Second value entered was not a number");
    
    let operator: char = operator.trim().chars().next().unwrap();
    match operator {
        '+' => println!("{} {} {} = {}", first_number, operator, second_number, first_number + second_number),
        '-' => println!("{} {} {} = {}", first_number, operator, second_number, first_number - second_number),
        '/' => println!("{} {} {} = {}", first_number, operator, second_number, first_number / second_number),
        '*' | 'x' | 'X' => println!("{} {} {} = {}", first_number, operator, second_number, first_number * second_number),
        _ => println!("Sorry, you entererd a wrong operator '{}' so your calculation was not processed", operator),
    };

}
