use restaurant::{
    front_of_house::{hosting::{self, Table, WaitList}},
    back_of_house::{cooking::{self, MealType, Breakfast}},
    serve_order,
};

fn main() {
    println!("Restaurant Management System Demo");
    println!("================================\n");

    // Demonstrate front of house operations
    println!("Front of House Operations:");
    println!("-----------------------");
    
    // Check table availability
    if hosting::check_availability() {
        println!("Tables are available!");
        
        // Add customers to waitlist
        let mut wait_list = WaitList::new();
        wait_list.add(String::from("Smith party"));
        
        // Seat customers
        hosting::seat_at_table();
    } else {
        println!("Sorry, all tables are occupied!");
    }

    // Demonstrate back of house operations
    println!("\nBack of House Operations:");
    println!("-----------------------");
    
    // Prepare different meals
    println!("\nPreparing meals:");
    cooking::cook_order(MealType::Breakfast);
    cooking::cook_order(MealType::Lunch);
    cooking::cook_order(MealType::Dinner);

    // Demonstrate breakfast customization
    println!("\nCustomizing breakfast order:");
    let mut breakfast = Breakfast::summer("Sourdough");
    breakfast.toast = String::from("Rye");  // We can modify toast because it's public
    println!("Modified breakfast toast to: {}", breakfast.toast);

    // Demonstrate the main library function
    println!("\nServing order:");
    serve_order();
} 