use rand::Rng;

#[derive(Debug)]
pub struct Table {
    number: u32,
    capacity: u32,
    occupied: bool,
}

impl Table {
    pub fn new(number: u32, capacity: u32) -> Table {
        Table {
            number,
            capacity,
            occupied: false,
        }
    }
}

// List of customers waiting to be seated
pub struct WaitList {
    customers: Vec<String>,
}

impl WaitList {
    pub fn new() -> WaitList {
        WaitList {
            customers: Vec::new(),
        }
    }

    pub fn add(&mut self, customer: String) {
        self.customers.push(customer);
    }

    pub fn next(&mut self) -> Option<String> {
        self.customers.pop()
    }
}

// Public functions for hosting
pub fn add_to_waitlist() {
    let mut wait_list = WaitList::new();
    wait_list.add(String::from("Customer 1"));
    println!("Added to waitlist!");
}

pub fn seat_at_table() {
    let table_number = rand::thread_rng().gen_range(1..20);
    let table = Table::new(table_number, 4);
    println!("Please follow me to table {}", table.number);
}

pub fn check_availability() -> bool {
    // Simulate checking table availability
    rand::thread_rng().gen_bool(0.5)
}

// Private helper function
fn clean_table(table: &mut Table) {
    println!("Cleaning table {}", table.number);
    table.occupied = false;
} 