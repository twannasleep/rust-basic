# Ownership and Borrowing Exercises

## 🌟 Exercise 1: String Manipulator

Create a program that demonstrates string ownership:

- Takes a string as input
- Provides functions to:
  - Reverse the string (without using .reverse())
  - Count words
  - Replace words
- Each function should demonstrate different ownership concepts

**Skills practiced:**

- String ownership
- Borrowing
- References
- String manipulation

## 🌟🌟 Exercise 2: Text Analyzer

Create a program that analyzes text:

- Reads text from a file
- Finds unique words (case insensitive)
- Tracks word frequency
- Returns slices of sentences
- Implements custom string type

**Skills practiced:**

- String slices
- Ownership
- References
- File handling

## 🌟🌟 Exercise 3: Custom Vector

Implement a simplified vector type that:

- Stores elements on the heap
- Implements:
  - push
  - pop
  - get (returns reference)
  - get_mut (returns mutable reference)
- Demonstrates proper memory management

**Skills practiced:**

- Memory management
- References
- Ownership
- Custom types

## 🌟🌟🌟 Exercise 4: Document Manager

Create a document management system that:

- Manages multiple documents
- Allows:
  - Reading documents (multiple readers)
  - Editing documents (single writer)
  - Moving documents between collections
- Implements proper borrowing rules

**Skills practiced:**

- Multiple ownership
- Mutable/immutable references
- Lifetime management
- Collections

## 🌟🌟🌟 Exercise 5: Memory Safe Data Structure

Implement a doubly-linked list that:

- Uses safe Rust (no unsafe code)
- Supports:
  - Insert
  - Remove
  - Iterate
- Handles ownership correctly
- Uses reference counting where appropriate

**Skills practiced:**

- Complex ownership
- Reference counting
- Safe memory management
- Data structures

## Tips

1. Pay attention to ownership transfer
2. Use references when full ownership isn't needed
3. Think about lifetimes
4. Consider using smart pointers
5. Test with different scenarios

## Common Pitfalls to Avoid

- Moving values unexpectedly
- Creating dangling references
- Fighting the borrow checker
- Using unsafe code unnecessarily

## Evaluation Criteria

- Memory safety
- Ownership clarity
- Reference usage
- Error handling
- Code organization

## Solutions

Solutions are in the `solutions` directory. Each solution includes:

- Implementation
- Tests
- Explanation of ownership concepts used
