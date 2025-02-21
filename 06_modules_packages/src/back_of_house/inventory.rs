use std::collections::HashMap;

// Private struct for inventory management
struct Inventory {
    supplies: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Inventory {
        let mut supplies = HashMap::new();
        supplies.insert(String::from("flour"), 100);
        supplies.insert(String::from("sugar"), 50);
        supplies.insert(String::from("eggs"), 144);
        Inventory { supplies }
    }

    fn check_stock(&self, item: &str) -> Option<&u32> {
        self.supplies.get(item)
    }

    fn remove_item(&mut self, item: &str, amount: u32) -> bool {
        if let Some(quantity) = self.supplies.get_mut(item) {
            if *quantity >= amount {
                *quantity -= amount;
                return true;
            }
        }
        false
    }
}

// Public functions that use the private Inventory struct
pub fn check_ingredient(ingredient: &str) -> bool {
    let inventory = Inventory::new();
    inventory.check_stock(ingredient).is_some()
}

pub fn remove_ingredients() {
    let mut inventory = Inventory::new();
    inventory.remove_item("flour", 5);
    println!("Removed ingredients from inventory");
}

// Private helper function
fn update_inventory_log() {
    println!("Updating inventory log...");
} 