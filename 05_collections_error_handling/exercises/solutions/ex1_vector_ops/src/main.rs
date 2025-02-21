fn main() {
    // 1. Creating and initializing vectors
    println!("1. Creating and initializing vectors:");
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    let mut vec_with_capacity = Vec::with_capacity(5);
    vec_with_capacity.push(10);
    vec_with_capacity.push(20);

    let vec_macro = vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);
    println!("vec_with_capacity: {:?}", vec_with_capacity);
    println!("vec_macro: {:?}", vec_macro);

    // 2. Adding and removing elements
    println!("\n2. Adding and removing elements:");
    numbers.push(4);
    println!("After push(4): {:?}", numbers);
    
    if let Some(last) = numbers.pop() {
        println!("Popped value: {}", last);
    }
    println!("After pop(): {:?}", numbers);

    numbers.insert(1, 10);
    println!("After insert(1, 10): {:?}", numbers);
    
    numbers.remove(1);
    println!("After remove(1): {:?}", numbers);

    // 3. Accessing and modifying elements
    println!("\n3. Accessing and modifying elements:");
    if let Some(first) = numbers.first() {
        println!("First element: {}", first);
    }
    
    if let Some(last) = numbers.last() {
        println!("Last element: {}", last);
    }

    if let Some(second) = numbers.get(1) {
        println!("Second element: {}", second);
    }

    if let Some(element) = numbers.get_mut(0) {
        *element *= 2;
    }
    println!("After doubling first element: {:?}", numbers);

    // 4. Iterating and transforming vectors
    println!("\n4. Iterating and transforming vectors:");
    
    // Iterate and print
    print!("Iterating: ");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // Map transformation
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Filter transformation
    let even: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", even);

    // 5. Vector methods and operations
    println!("\n5. Vector methods and operations:");
    numbers.extend(vec![5, 6, 7]);
    println!("After extend: {:?}", numbers);

    numbers.sort();
    println!("After sort: {:?}", numbers);

    numbers.dedup();
    println!("After dedup: {:?}", numbers);

    println!("Length: {}", numbers.len());
    println!("Capacity: {}", numbers.capacity());
    println!("Is empty: {}", numbers.is_empty());

    // Clear the vector
    numbers.clear();
    println!("After clear: {:?}", numbers);
}
