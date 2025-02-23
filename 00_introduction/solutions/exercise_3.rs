fn main() {
    // Integer overflow
    let max_u8 = u8::MAX;
    println!("u8 max: {}", max_u8);
    // println!("Overflow: {}", max_u8 + 1); // This would panic in debug mode
    
    // Type conversion
    let x = 13u8;
    let y = x as u32 * 2;
    println!("Converted: {}", y);
    
    // Boolean logic
    let has_access = true;
    let knows_password = false;
    println!("Access granted: {}", has_access && knows_password);
} 