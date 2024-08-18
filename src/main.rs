use std::io;

/// main function that repeatedly prompts the user to enter numbers and an opeeratop to perform calculations.
fn main() {
   


    loop {
        println!("Enter first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");

        println!("Enter an operator (+, -, *, /):");
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");

        let num1: f64 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        let result = match operator.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Cannot divide by zero");
                    continue;
                } else {
                    num1 / num2
                }
            },
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("The result is: {}", result);
        
        println!("Do you want to perform another calculation? (yes/no)");
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read line");

        if response.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }
}


