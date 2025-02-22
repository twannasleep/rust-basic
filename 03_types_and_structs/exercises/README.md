# Types and Structs Exercises

## Exercise 1: Game Character System

Create a game character system with the following requirements:

1. Define a `Character` struct with:
   - Name
   - Health points
   - Level
   - Character class (using an enum)
   - Inventory (using a Vec)

2. Implement the following traits:
   - `Attack`: Calculate damage based on level and class
   - `Defend`: Calculate defense based on equipment
   - `LevelUp`: Handle level-up mechanics

Example structure:
```rust
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

struct Character {
    name: String,
    health: u32,
    level: u32,
    class: CharacterClass,
    inventory: Vec<Item>,
}

trait Attack {
    fn calculate_damage(&self) -> u32;
}

// TODO: Implement the system
```

## Exercise 2: Smart Home Devices

Create a smart home system using enums and traits:

1. Define different device types using enums
2. Implement a common `Device` trait
3. Handle device states and commands
4. Implement error handling for invalid operations

Example structure:
```rust
enum DeviceType {
    Light { brightness: u8 },
    Thermostat { temperature: f64 },
    Lock { locked: bool },
}

trait Device {
    fn turn_on(&mut self) -> Result<(), String>;
    fn turn_off(&mut self) -> Result<(), String>;
    fn get_status(&self) -> String;
}

// TODO: Implement the system
```

## Exercise 3: Generic Data Container

Implement a generic data container with the following features:

1. Store any type that implements `Clone` and `Debug`
2. Provide methods for:
   - Adding items
   - Removing items
   - Transforming items
   - Filtering items
3. Implement custom iterators

Example structure:

```rust
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T: Clone + Debug> Container<T> {
    fn new() -> Self {
        // TODO: Implement constructor
    }
    
    fn transform<F>(&self, f: F) -> Container<T>
    where
        F: Fn(&T) -> T
    {
        // TODO: Implement transformation
    }
}
```

## Exercise 4: Custom Error Types

Create a library management system with custom error handling:

1. Define custom error types using enums
2. Implement the `Error` and `Display` traits
3. Handle multiple error cases
4. Use the `Result` type appropriately

Example structure:

```rust
#[derive(Debug)]
enum LibraryError {
    BookNotFound(String),
    AlreadyBorrowed(String),
    InvalidInput { details: String },
}

impl std::error::Error for LibraryError {}

impl Display for LibraryError {
    // TODO: Implement display formatting
}

struct Library {
    // TODO: Implement library system
}
```

## Bonus Challenge: Expression Evaluator

Create an expression evaluator that:

1. Uses enums to represent different types of expressions:
   - Numbers
   - Variables
   - Binary operations
   - Function calls

2. Implements evaluation using pattern matching
3. Handles operator precedence
4. Provides error handling for invalid expressions

Example structure:

```rust
enum Expression {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Expression {
    fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, String> {
        // TODO: Implement expression evaluation
    }
}
```

## Evaluation Criteria

Your solutions will be evaluated based on:

1. Proper use of Rust's type system
2. Implementation of traits and generics
3. Error handling
4. Code organization and documentation
5. Test coverage

## Testing

For each exercise:

1. Write unit tests for all functionality
2. Test edge cases and error conditions
3. Document test cases
4. Use test-driven development approach
