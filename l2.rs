use std::{f64, io};

fn int_check(x: &str) -> bool {
    x.parse::<f64>().is_ok()
}

fn main() {
    let operators = ["+", "-", "/", "*"];
    let mut exitcheck = true;
    let mut last_result: Option<f64> = None;

    while exitcheck {
        println!("First digit (or type 'prev' to use the last result):");

        let mut digit1 = String::new();
        io::stdin().read_line(&mut digit1).expect("failed to read");
        let digit1 = digit1.trim();

        let value1: f64 = if digit1 == "prev" {
            if let Some(result) = last_result {
                println!("Using previous result: {}", result);
                result
            } else {
                println!("No previous result available. Please enter a valid number.");
                continue;
            }
        } else if digit1 == "exit" {
            println!("Goodbye");
            exitcheck = false;
            break;
        } else if int_check(digit1) {
            digit1.parse().unwrap()
        } else {
            println!("Invalid input. Please enter a valid number or 'prev'.");
            continue;
        };

        println!("Operator (+, -, /, *):");
        let mut sign = String::new();
        io::stdin().read_line(&mut sign).expect("failed to read");

        while !operators.contains(&sign.trim()) {
            println!("Invalid operator. Enter a valid one (+, -, /, *):");
            sign.clear();
            io::stdin().read_line(&mut sign).expect("failed to read");
        }

        println!("Second digit:");
        let mut digit2 = String::new();
        io::stdin().read_line(&mut digit2).expect("failed to read");
        let digit2 = digit2.trim();

        if digit2 == "exit" {
            println!("Goodbye");
            exitcheck = false;
        } else if digit2 == "prev" {
            if let Some(result) = last_result {
                println!("Using previous result: {}", result);
                let value2 = result;

                let result = match sign.trim() {
                    "+" => value1 + value2,
                    "-" => value1 - value2,
                    "*" => value1 * value2,
                    "/" => {
                        if value2 != 0.0 {
                            value1 / value2
                        } else {
                            println!("Error: Division by zero");
                            continue;
                        }
                    }
                    _ => {
                        println!("Unknown operator.");
                        continue;
                    }
                };

                println!("Result: {}", result);
                last_result = Some(result);
            } else {
                println!("No previous result available. Please enter a valid number.");
                continue;
            }
        } else if int_check(digit2) {
            let value2: f64 = digit2.parse().unwrap();

            let result = match sign.trim() {
                "+" => value1 + value2,
                "-" => value1 - value2,
                "*" => value1 * value2,
                "/" => {
                    if value2 != 0.0 {
                        value1 / value2
                    } else {
                        println!("Error: Division by zero");
                        continue;
                    }
                }
                _ => {
                    println!("Unknown operator.");
                    continue;
                }
            };

            println!("Result: {}", result);
            last_result = Some(result);
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}