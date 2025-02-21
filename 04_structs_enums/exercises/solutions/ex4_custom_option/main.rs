// Custom Option type
#[derive(Debug, Clone, PartialEq)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

// Implementation of common Option-like methods
impl<T> Maybe<T> {
    // Constructor for Some value
    fn just(value: T) -> Self {
        Maybe::Just(value)
    }

    // Constructor for None value
    fn nothing() -> Self {
        Maybe::Nothing
    }

    // Returns true if the option is a Just value
    fn is_just(&self) -> bool {
        matches!(self, Maybe::Just(_))
    }

    // Returns true if the option is Nothing
    fn is_nothing(&self) -> bool {
        matches!(self, Maybe::Nothing)
    }

    // Transforms the Maybe<T> into Option<T>
    fn into_option(self) -> Option<T> {
        match self {
            Maybe::Just(value) => Some(value),
            Maybe::Nothing => None,
        }
    }

    // Maps a Maybe<T> to Maybe<U> by applying a function to the contained value
    fn map<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Maybe::Just(value) => Maybe::Just(f(value)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    // Returns the contained Just value or panics
    fn unwrap(self) -> T
    where
        T: std::fmt::Debug,
    {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => panic!("Called unwrap on a Nothing value"),
        }
    }

    // Returns the contained Just value or a default
    fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => default,
        }
    }

    // Returns the contained Just value or computes it from a closure
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => f(),
        }
    }

    // Returns the contained Just value or the parameter
    fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => T::default(),
        }
    }

    // Applies a function to the contained value (if any) and returns the result
    fn and_then<U, F>(self, f: F) -> Maybe<U>
    where
        F: FnOnce(T) -> Maybe<U>,
    {
        match self {
            Maybe::Just(value) => f(value),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    // Returns Nothing if the option is Nothing, otherwise returns opt
    fn and<U>(self, opt: Maybe<U>) -> Maybe<U> {
        match self {
            Maybe::Just(_) => opt,
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    // Returns Nothing if the option is Just, otherwise returns opt
    fn or(self, opt: Maybe<T>) -> Maybe<T> {
        match self {
            Maybe::Just(value) => Maybe::Just(value),
            Maybe::Nothing => opt,
        }
    }

    // Returns the option if it contains a value, otherwise returns the result of calling f
    fn or_else<F>(self, f: F) -> Maybe<T>
    where
        F: FnOnce() -> Maybe<T>,
    {
        match self {
            Maybe::Just(value) => Maybe::Just(value),
            Maybe::Nothing => f(),
        }
    }

    // Transforms the Maybe<T> into a Result<T, E>
    fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Maybe::Just(value) => Ok(value),
            Maybe::Nothing => Err(err),
        }
    }

    // Transforms the Maybe<T> into a Result<T, E> using a closure
    fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Maybe::Just(value) => Ok(value),
            Maybe::Nothing => Err(err()),
        }
    }
}

// Example usage and tests
fn main() {
    // Basic usage
    let just_value = Maybe::just(42);
    let nothing_value: Maybe<i32> = Maybe::nothing();

    println!("just_value: {:?}", just_value);
    println!("nothing_value: {:?}", nothing_value);

    // Testing map
    let mapped = just_value.clone().map(|x| x * 2);
    println!("mapped: {:?}", mapped);

    // Testing unwrap_or
    let unwrapped = nothing_value.clone().unwrap_or(0);
    println!("unwrapped with default: {}", unwrapped);

    // Testing and_then
    let and_then_result = just_value
        .clone()
        .and_then(|x| if x > 0 { Maybe::just(x + 1) } else { Maybe::nothing() });
    println!("and_then result: {:?}", and_then_result);

    // Example with string manipulation
    let name = Maybe::just("John");
    let greeting = name
        .clone()
        .map(|n| format!("Hello, {}!", n))
        .unwrap_or_else(|| "Hello, stranger!".to_string());
    println!("{}", greeting);

    // Chaining operations
    let complex = Maybe::just(10)
        .and_then(|x| if x > 5 { Maybe::just(x * 2) } else { Maybe::nothing() })
        .map(|x| x + 1)
        .and_then(|x| if x % 2 == 0 { Maybe::nothing() } else { Maybe::just(x) });
    println!("Complex chain result: {:?}", complex);

    // Converting to Result
    let result: Result<i32, &str> = Maybe::just(42).ok_or("No value");
    println!("Result: {:?}", result);

    // Working with Option conversion
    let opt: Option<i32> = Maybe::just(123).into_option();
    println!("Converted to Option: {:?}", opt);

    // Example with custom type
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Maybe::just(Person {
        name: "Alice".to_string(),
        age: 30,
    });

    let age_next_year = person.map(|p| p.age + 1);
    println!("Age next year: {:?}", age_next_year);

    // Example with error handling
    fn divide(a: i32, b: i32) -> Maybe<i32> {
        if b == 0 {
            Maybe::nothing()
        } else {
            Maybe::just(a / b)
        }
    }

    let division_result = divide(10, 2)
        .and_then(|x| divide(x, 0))
        .unwrap_or(-1);
    println!("Division result: {}", division_result);
} 