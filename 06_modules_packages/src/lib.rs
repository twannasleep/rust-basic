// Restaurant management system

// Declare modules
pub mod front_of_house;
pub mod back_of_house;

// Re-export commonly used items
pub use crate::front_of_house::hosting;
pub use crate::back_of_house::cooking;

// Main entry point for the restaurant functionality
pub fn serve_order() {
    // Using functions from different modules
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    
    let mut meal = back_of_house::cooking::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // We can't modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries"); // This won't compile
} 