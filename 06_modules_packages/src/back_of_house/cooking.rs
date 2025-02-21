use rand::Rng;

// Public struct with some private fields
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// Private struct for internal use
struct Recipe {
    ingredients: Vec<String>,
    preparation_time: u32,
}

impl Recipe {
    fn new() -> Recipe {
        Recipe {
            ingredients: Vec::new(),
            preparation_time: 0,
        }
    }
}

// Public enum for meal types
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
}

// Public functions
pub fn cook_order(meal: MealType) {
    match meal {
        MealType::Breakfast => prepare_breakfast(),
        MealType::Lunch => prepare_lunch(),
        MealType::Dinner => prepare_dinner(),
    }
}

pub fn fix_incorrect_order() {
    cook_order(MealType::Dinner);
    super::inventory::remove_ingredients();
}

// Private functions
fn prepare_breakfast() {
    let recipe = get_breakfast_recipe();
    println!("Preparing breakfast for {} minutes", recipe.preparation_time);
}

fn prepare_lunch() {
    let recipe = get_lunch_recipe();
    println!("Preparing lunch for {} minutes", recipe.preparation_time);
}

fn prepare_dinner() {
    let recipe = get_dinner_recipe();
    println!("Preparing dinner for {} minutes", recipe.preparation_time);
}

fn get_breakfast_recipe() -> Recipe {
    let mut recipe = Recipe::new();
    recipe.ingredients = vec![
        String::from("eggs"),
        String::from("bread"),
        String::from("bacon"),
    ];
    recipe.preparation_time = rand::thread_rng().gen_range(10..20);
    recipe
}

fn get_lunch_recipe() -> Recipe {
    let mut recipe = Recipe::new();
    recipe.ingredients = vec![
        String::from("chicken"),
        String::from("rice"),
        String::from("vegetables"),
    ];
    recipe.preparation_time = rand::thread_rng().gen_range(15..30);
    recipe
}

fn get_dinner_recipe() -> Recipe {
    let mut recipe = Recipe::new();
    recipe.ingredients = vec![
        String::from("steak"),
        String::from("potatoes"),
        String::from("salad"),
    ];
    recipe.preparation_time = rand::thread_rng().gen_range(20..40);
    recipe
} 