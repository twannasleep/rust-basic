fn main() {
    // BMI calculation
    let weight_kg = 75.0;
    let height_m = 1.80;
    let bmi = weight_kg / height_m.powi(2);
    
    // Username generation
    let first_name = "Ferris";
    let last_name = "Crab";
    let username = format!("{}_{}", first_name.to_lowercase(), last_name.to_lowercase());
    
    // Formatted output
    println!("=== Health Stats ===");
    println!("Name: {} {}", first_name, last_name);
    println!("Username: {}", username);
    println!("BMI: {:.1}", bmi);
} 