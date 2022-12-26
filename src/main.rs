use std::io;
use colored::*;

fn main() {
    println!("{}", "Hello, world!".green().bold());
    println!("{}", "Enter some numbers separated by spaces:".red().bold());
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("Failed to read line");
    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    println!("Enter an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();
    let result = match operation {
        "+" => add(&numbers),
        "-" => subtract(&numbers),
        "*" => multiply(&numbers),
        "/" => divide(&numbers),
        _ => panic!("Invalid operation"),
    };
    println!("{} {}","Result: ".green(), result.to_string().trim().red().bold());
    // wait for user input
    println!("{}", "Press enter to exit...".green().bold());
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}

fn add(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

fn subtract(numbers: &[i32]) -> i32 {
    let mut difference = 0;
    for number in numbers {
        difference -= number;
    }
    difference
}

fn multiply(numbers: &[i32]) -> i32 {
    let mut product = 1;
    for number in numbers {
        product *= number;
    }
    product
}

fn divide(numbers: &[i32]) -> i32 {
    let mut quotient = 1;
    for number in numbers {
        quotient /= number;
    }
    quotient
}