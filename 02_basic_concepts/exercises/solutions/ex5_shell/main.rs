use std::io::{self, Write};
use std::collections::VecDeque;

const MAX_HISTORY: usize = 10;

struct Shell {
    history: VecDeque<String>,
    running: bool,
}

impl Shell {
    fn new() -> Self {
        Shell {
            history: VecDeque::with_capacity(MAX_HISTORY),
            running: true,
        }
    }

    fn add_to_history(&mut self, command: String) {
        if self.history.len() >= MAX_HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(command);
    }

    fn execute_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "exit" => {
                println!("Goodbye!");
                self.running = false;
            }
            "help" => {
                self.show_help();
            }
            "echo" => {
                println!("{}", &command[5..]);
            }
            "history" => {
                self.show_history();
            }
            "calc" => {
                if parts.len() != 4 {
                    println!("Usage: calc <number> <operator> <number>");
                    println!("Operators: +, -, *, /");
                    return;
                }
                self.calculate(&parts[1..]);
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", parts[0]);
            }
        }
    }

    fn calculate(&self, args: &[&str]) {
        let num1: f64 = match args[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: First argument must be a number");
                return;
            }
        };

        let num2: f64 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Third argument must be a number");
                return;
            }
        };

        let result = match args[1] {
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

        println!("{} {} {} = {}", num1, args[1], num2, result);
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  help              - Show this help message");
        println!("  echo <text>       - Print the text");
        println!("  calc n1 op n2     - Calculate: n1 op n2 (op: +, -, *, /)");
        println!("  history           - Show command history");
        println!("  exit              - Exit the shell");
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No commands in history.");
            return;
        }
        println!("Command history:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, cmd);
        }
    }

    fn run(&mut self) {
        println!("Welcome to Mini Shell!");
        println!("Type 'help' for available commands.");

        while self.running {
            print!("myshell> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input!");
                continue;
            }

            let command = input.trim().to_string();
            if !command.is_empty() {
                self.add_to_history(command.clone());
                self.execute_command(&command);
            }
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.run();
} 