use std::io;

fn main() {
    // Start an infinite loop for the calculator application
    loop {
        println!("\n--- Calculator ---");
        println!("What operation do you want to complete? ('+', '-', '*' or '/')");
        println!("(Or type 'exit' / 'q' to quit)");

        // Create a mutable String buffer to hold the user's operation choice
        let mut operation: String = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line.");

        // Clean up the string input by trimming hidden trailing newline characters (\n)
        let operation_trimmed = operation.trim();

        // Check if the user wants to leave the application before asking for numbers
        if operation_trimmed == "exit" || operation_trimmed == "q" {
            println!("Goodbye!");
            break; // Exits the infinite loop gracefully
        }

        println!("Enter number 1");
        let mut num1: String = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line.");

        // Strip whitespace and attempt to parse the string into a 32-bit float (f32)
        let num1: f32 = match num1.trim().parse() {
            Ok(num) => num, // Success: bind the parsed number to num1
            Err(_) => {     // Error: notify the user and restart the loop from the top
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("Enter number 2");
        let mut num2: String = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line.");

        // Strip whitespace and parse the second number into a 32-bit float (f32)
        let num2: f32 = match num2.trim().parse() {
            Ok(num) => num, // Success: bind the parsed number to num2
            Err(_) => {     // Error: notify the user and restart the loop from the top
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Match against the trimmed string slice to execute the chosen calculation
        match operation_trimmed {
            "+" => println!("Result: {}", num1 + num2),
            "-" => println!("Result: {}", num1 - num2),
            "*" => println!("Result: {}", num1 * num2),
            "/" => {
                // Safeguard against runtime crashes or undefined math errors
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                } else {
                    println!("Result: {}", num1 / num2);
                }
            }
            _ => {
                // Catch-all case for any unrecognized operator strings
                println!("Invalid input: '{}'", operation_trimmed);
                continue;
            }
        }
    }
}
