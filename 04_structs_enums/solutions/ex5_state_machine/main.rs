use std::collections::HashMap;
use std::io::{self, Write};

// Product struct
#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: u32,
    quantity: u32,
}

// Money representation in cents
#[derive(Debug, Clone, Copy)]
struct Money(u32);

impl Money {
    fn format(&self) -> String {
        format!("${:.2}", self.0 as f64 / 100.0)
    }
}

// Vending machine states
#[derive(Debug)]
enum State {
    Ready,
    AwaitingPayment {
        product: Product,
        amount_paid: Money,
    },
    Dispensing {
        product: Product,
        change: Money,
    },
    OutOfStock {
        product: String,
    },
}

// Vending machine errors
#[derive(Debug)]
enum VendingError {
    InvalidProduct(String),
    InsufficientPayment(Money),
    OutOfStock(String),
    InvalidMoney(String),
}

// Vending machine struct
struct VendingMachine {
    state: State,
    products: HashMap<String, Product>,
    money_collected: Money,
}

impl VendingMachine {
    fn new() -> Self {
        let mut products = HashMap::new();
        products.insert(
            "A1".to_string(),
            Product {
                name: "Cola".to_string(),
                price: 150, // $1.50
                quantity: 5,
            },
        );
        products.insert(
            "A2".to_string(),
            Product {
                name: "Chips".to_string(),
                price: 100, // $1.00
                quantity: 3,
            },
        );
        products.insert(
            "A3".to_string(),
            Product {
                name: "Candy".to_string(),
                price: 75, // $0.75
                quantity: 0, // Out of stock
            },
        );

        VendingMachine {
            state: State::Ready,
            products,
            money_collected: Money(0),
        }
    }

    fn select_product(&mut self, code: &str) -> Result<(), VendingError> {
        match &self.state {
            State::Ready => {
                let product = self
                    .products
                    .get(code)
                    .ok_or_else(|| VendingError::InvalidProduct(code.to_string()))?
                    .clone();

                if product.quantity == 0 {
                    self.state = State::OutOfStock {
                        product: product.name.clone(),
                    };
                    return Err(VendingError::OutOfStock(product.name));
                }

                self.state = State::AwaitingPayment {
                    product,
                    amount_paid: Money(0),
                };
                Ok(())
            }
            _ => Err(VendingError::InvalidProduct(
                "Machine is not ready for selection".to_string(),
            )),
        }
    }

    fn insert_money(&mut self, amount: u32) -> Result<(), VendingError> {
        match &mut self.state {
            State::AwaitingPayment {
                product,
                amount_paid,
            } => {
                amount_paid.0 += amount;
                
                if amount_paid.0 >= product.price {
                    let change = Money(amount_paid.0 - product.price);
                    let product = product.clone();
                    
                    // Update product quantity
                    if let Some(p) = self.products.get_mut(&product.name) {
                        p.quantity -= 1;
                    }

                    // Collect money
                    self.money_collected.0 += product.price;

                    // Transition to dispensing state
                    self.state = State::Dispensing { product, change };
                }
                
                Ok(())
            }
            _ => Err(VendingError::InvalidMoney(
                "Machine is not awaiting payment".to_string(),
            )),
        }
    }

    fn complete_transaction(&mut self) -> Result<(), VendingError> {
        match &self.state {
            State::Dispensing { product, change } => {
                println!("Dispensing {}...", product.name);
                if change.0 > 0 {
                    println!("Returning change: {}", change.format());
                }
                self.state = State::Ready;
                Ok(())
            }
            State::OutOfStock { product } => {
                println!("{} is out of stock!", product);
                self.state = State::Ready;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn cancel_transaction(&mut self) -> Result<Money, VendingError> {
        match &self.state {
            State::AwaitingPayment { amount_paid, .. } => {
                let refund = *amount_paid;
                self.state = State::Ready;
                Ok(refund)
            }
            _ => Ok(Money(0)),
        }
    }

    fn display_status(&self) {
        println!("\nVending Machine Status:");
        println!("----------------------");
        println!("State: {:?}", self.state);
        println!("Money collected: {}", self.money_collected.format());
        println!("\nAvailable Products:");
        for (code, product) in &self.products {
            println!(
                "{}: {} - {} ({} available)",
                code,
                product.name,
                Money(product.price).format(),
                product.quantity
            );
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_money(input: &str) -> Result<u32, String> {
    let amount = input.parse::<f64>().map_err(|_| "Invalid amount".to_string())?;
    Ok((amount * 100.0) as u32)
}

fn main() {
    let mut machine = VendingMachine::new();

    println!("Vending Machine Simulator");
    println!("------------------------");

    loop {
        println!("\nOptions:");
        println!("1. Display products and status");
        println!("2. Select product");
        println!("3. Insert money");
        println!("4. Cancel transaction");
        println!("5. Quit");

        let choice = get_user_input("\nSelect option (1-5): ");

        match choice.as_str() {
            "1" => machine.display_status(),
            "2" => {
                let code = get_user_input("Enter product code: ");
                match machine.select_product(&code) {
                    Ok(_) => println!("Product selected. Please insert money."),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "3" => {
                let amount_str = get_user_input("Enter amount (in dollars): ");
                match parse_money(&amount_str) {
                    Ok(amount) => {
                        match machine.insert_money(amount) {
                            Ok(_) => {
                                // Try to complete the transaction
                                if let Err(e) = machine.complete_transaction() {
                                    println!("Error completing transaction: {:?}", e);
                                }
                            }
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                match machine.cancel_transaction() {
                    Ok(refund) => {
                        if refund.0 > 0 {
                            println!("Refunding {}", refund.format());
                        }
                        println!("Transaction cancelled.");
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select 1-5."),
        }
    }
} 