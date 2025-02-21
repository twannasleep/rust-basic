use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Node structure for the doubly-linked list
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

// Doubly-linked list structure
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Insert at the end of the list
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(old_tail) => {
                // Set up the new node's prev pointer
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                // Set up the old tail's next pointer
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                // Update the tail
                self.tail = Some(new_node);
            }
            None => {
                // Empty list
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.length += 1;
    }

    // Insert at the beginning of the list
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.head.take() {
            Some(old_head) => {
                // Set up the new node's next pointer
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                // Set up the old head's prev pointer
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                // Update the head
                self.head = Some(new_node);
            }
            None => {
                // Empty list
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.length += 1;
    }

    // Remove from the end of the list
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.length -= 1;

            match old_tail.borrow_mut().prev.take() {
                Some(prev) => {
                    // Update the new tail
                    if let Some(prev_strong) = prev.upgrade() {
                        prev_strong.borrow_mut().next = None;
                        self.tail = Some(prev_strong);
                    }
                }
                None => {
                    // List is now empty
                    self.head = None;
                }
            }

            // Extract the value
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }

    // Remove from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.length -= 1;

            match old_head.borrow_mut().next.take() {
                Some(next) => {
                    // Update the new head
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }
                None => {
                    // List is now empty
                    self.tail = None;
                }
            }

            // Extract the value
            Rc::try_unwrap(old_head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }

    // Iterator that yields immutable references
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(Rc::clone),
        }
    }
}

// Iterator implementation
pub struct Iter<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.borrow().next.as_ref().map(Rc::clone);
            node
        })
    }
}

// Example usage and tests
fn main() {
    // Create a new list
    let mut list = DoublyLinkedList::new();

    // Test pushing elements
    println!("Testing push operations:");
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(3);

    println!("List length: {}", list.len());
    print!("List contents: ");
    for node in list.iter() {
        print!("{} ", node.borrow().value);
    }
    println!();

    // Test popping elements
    println!("\nTesting pop operations:");
    println!("Popped from front: {:?}", list.pop_front());
    println!("Popped from back: {:?}", list.pop_back());
    
    print!("List contents after popping: ");
    for node in list.iter() {
        print!("{} ", node.borrow().value);
    }
    println!();
    println!("List length: {}", list.len());

    // Test with strings
    println!("\nTesting with strings:");
    let mut string_list = DoublyLinkedList::new();
    string_list.push_back(String::from("Hello"));
    string_list.push_back(String::from("World"));
    string_list.push_front(String::from("Greetings"));

    print!("String list contents: ");
    for node in string_list.iter() {
        print!("{} ", node.borrow().value);
    }
    println!();

    // Pop all elements
    println!("\nPopping all elements:");
    while let Some(value) = string_list.pop_back() {
        println!("Popped: {}", value);
    }
    println!("List is empty: {}", string_list.is_empty());
} 