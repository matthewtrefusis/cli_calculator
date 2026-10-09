use std::io;

fn main() {
    loop {
        println!("\n--- Calculator ---");
        println!("What operation do you want to complete? ('+', '-', '*', '/', '^' [power], 'v' [sqrt])");
        println!("(Or type 'exit' / 'q' to quit)");

        let mut operation: String = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line.");

        let operation_trimmed = operation.trim();

        if operation_trimmed == "exit" || operation_trimmed == "q" {
            println!("Goodbye!");
            break;
        }

        println!("Enter number 1 (or the base number)");
        let mut num1: String = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line.");

        let num1: f32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // NEW: Check for square root early because it only needs one number
        if operation_trimmed == "v" {
            if num1 < 0.0 {
                println!("Error: Cannot calculate the square root of a negative number!");
            } else {
                println!("Result: {}", num1.sqrt()); // Uses Rust's built-in .sqrt()
            }
            continue; // Skip the rest of the loop and start over
        }

        // If it's not a square root, ask for the second number as usual
        println!("Enter number 2 (or the exponent)");
        let mut num2: String = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line.");

        let num2: f32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match operation_trimmed {
            "+" => println!("Result: {}", num1 + num2),
            "-" => println!("Result: {}", num1 - num2),
            "*" => println!("Result: {}", num1 * num2),
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                } else {
                    println!("Result: {}", num1 / num2);
                }
            }
            "^" => println!("Result: {}", num1.powf(num2)), // Uses Rust's built-in .powf()
            _ => {
                println!("Invalid input: '{}'", operation_trimmed);
                continue;
            }
        }
    }
}
