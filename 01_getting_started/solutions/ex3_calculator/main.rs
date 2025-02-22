use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} <number1> <operator> <number2>", args[0]);
        println!("Supported operators: +, -, *, /");
        return;
    }

    let num1: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: First argument must be a number");
            return;
        }
    };

    let operator = &args[2];

    let num2: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Third argument must be a number");
            return;
        }
    };

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Error: Invalid operator. Use +, -, *, or /");
            return;
        }
    };

    println!("{} {} {} = {:.2}", num1, operator, num2, result);
} 