use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

// Struct to represent a product
#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

// Struct to represent a shopping cart
#[derive(Debug)]
struct ShoppingCart {
    items: HashMap<String, Product>,
    total_items: u32,
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart {
            items: HashMap::new(),
            total_items: 0,
        }
    }

    // Add or update item in cart
    fn add_item(&mut self, product: Product) {
        let id = product.name.to_lowercase();
        
        // Using entry API for clean insertion/update
        match self.items.entry(id) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                self.total_items += product.quantity;
                entry.insert(product);
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let existing = entry.get_mut();
                existing.quantity += product.quantity;
                self.total_items += product.quantity;
            }
        }
    }

    // Remove item from cart
    fn remove_item(&mut self, name: &str) -> Option<Product> {
        let id = name.to_lowercase();
        if let Some(product) = self.items.remove(&id) {
            self.total_items -= product.quantity;
            Some(product)
        } else {
            None
        }
    }

    // Update quantity of an item
    fn update_quantity(&mut self, name: &str, quantity: u32) -> bool {
        let id = name.to_lowercase();
        if let Some(product) = self.items.get_mut(&id) {
            self.total_items = self.total_items - product.quantity + quantity;
            product.quantity = quantity;
            true
        } else {
            false
        }
    }

    // Calculate total price
    fn total_price(&self) -> f64 {
        self.items.values()
            .map(|product| product.price * product.quantity as f64)
            .sum()
    }

    // Get most expensive item
    fn most_expensive_item(&self) -> Option<&Product> {
        self.items.values()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    }

    // Get items sorted by quantity
    fn items_by_quantity(&self) -> Vec<&Product> {
        let mut items: Vec<&Product> = self.items.values().collect();
        items.sort_by_key(|p| std::cmp::Reverse(p.quantity));
        items
    }
}

// Inventory management system
struct Inventory {
    products: HashMap<String, Product>,
    categories: HashMap<String, Vec<String>>, // Category -> Product names
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    // Add product with category
    fn add_product(&mut self, product: Product, category: &str) {
        let id = product.name.to_lowercase();
        
        // Add to products
        self.products.insert(id.clone(), product);
        
        // Add to category
        self.categories
            .entry(category.to_lowercase())
            .or_insert_with(Vec::new)
            .push(id);
    }

    // Get products by category
    fn get_by_category(&self, category: &str) -> Vec<&Product> {
        self.categories
            .get(&category.to_lowercase())
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.products.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    // Search products by name (partial match)
    fn search(&self, query: &str) -> Vec<&Product> {
        let query = query.to_lowercase();
        self.products
            .values()
            .filter(|p| p.name.to_lowercase().contains(&query))
            .collect()
    }

    // Get product statistics
    fn get_statistics(&self) -> HashMap<String, (u32, f64)> {
        let mut stats = HashMap::new();
        
        for (category, products) in &self.categories {
            let total_quantity: u32 = products
                .iter()
                .filter_map(|id| self.products.get(id))
                .map(|p| p.quantity)
                .sum();
                
            let total_value: f64 = products
                .iter()
                .filter_map(|id| self.products.get(id))
                .map(|p| p.price * p.quantity as f64)
                .sum();
                
            stats.insert(category.clone(), (total_quantity, total_value));
        }
        
        stats
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("HashMap Operations Demo");
    println!("---------------------\n");

    // Initialize inventory with some products
    let mut inventory = Inventory::new();
    
    // Add sample products
    inventory.add_product(
        Product {
            name: "Apple".to_string(),
            price: 0.50,
            quantity: 100,
        },
        "Fruits"
    );
    
    inventory.add_product(
        Product {
            name: "Banana".to_string(),
            price: 0.30,
            quantity: 150,
        },
        "Fruits"
    );
    
    inventory.add_product(
        Product {
            name: "Carrot".to_string(),
            price: 0.25,
            quantity: 200,
        },
        "Vegetables"
    );
    
    inventory.add_product(
        Product {
            name: "Milk".to_string(),
            price: 2.99,
            quantity: 50,
        },
        "Dairy"
    );

    // Create a shopping cart
    let mut cart = ShoppingCart::new();

    // Demo loop
    loop {
        println!("\nOptions:");
        println!("1. View inventory by category");
        println!("2. Search products");
        println!("3. Add to cart");
        println!("4. View cart");
        println!("5. Update cart quantity");
        println!("6. Remove from cart");
        println!("7. View inventory statistics");
        println!("8. Quit");

        let choice = get_user_input("\nSelect option (1-8): ");

        match choice.as_str() {
            "1" => {
                let category = get_user_input("Enter category (Fruits/Vegetables/Dairy): ");
                let products = inventory.get_by_category(&category);
                println!("\nProducts in {}:", category);
                for product in products {
                    println!("- {} (${:.2}, {} in stock)", 
                        product.name, product.price, product.quantity);
                }
            }
            "2" => {
                let query = get_user_input("Enter search term: ");
                let results = inventory.search(&query);
                println!("\nSearch results:");
                for product in results {
                    println!("- {} (${:.2}, {} in stock)",
                        product.name, product.price, product.quantity);
                }
            }
            "3" => {
                let name = get_user_input("Enter product name: ");
                let quantity: u32 = get_user_input("Enter quantity: ")
                    .parse()
                    .unwrap_or(0);
                
                if let Some(product) = inventory.products.get(&name.to_lowercase()) {
                    let mut new_product = product.clone();
                    new_product.quantity = quantity;
                    cart.add_item(new_product);
                    println!("Added to cart!");
                } else {
                    println!("Product not found!");
                }
            }
            "4" => {
                println!("\nShopping Cart:");
                println!("-------------");
                for product in cart.items.values() {
                    println!("- {} x{} (${:.2} each)",
                        product.name, product.quantity, product.price);
                }
                println!("\nTotal items: {}", cart.total_items);
                println!("Total price: ${:.2}", cart.total_price());
                
                if let Some(most_expensive) = cart.most_expensive_item() {
                    println!("Most expensive item: {} (${:.2})",
                        most_expensive.name, most_expensive.price);
                }
            }
            "5" => {
                let name = get_user_input("Enter product name: ");
                let quantity: u32 = get_user_input("Enter new quantity: ")
                    .parse()
                    .unwrap_or(0);
                
                if cart.update_quantity(&name, quantity) {
                    println!("Quantity updated!");
                } else {
                    println!("Product not found in cart!");
                }
            }
            "6" => {
                let name = get_user_input("Enter product name to remove: ");
                if let Some(product) = cart.remove_item(&name) {
                    println!("Removed {} from cart!", product.name);
                } else {
                    println!("Product not found in cart!");
                }
            }
            "7" => {
                println!("\nInventory Statistics:");
                println!("--------------------");
                let stats = inventory.get_statistics();
                for (category, (quantity, value)) in stats {
                    println!("{}: {} items, total value: ${:.2}",
                        category, quantity, value);
                }
            }
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select 1-8."),
        }
    }
}
