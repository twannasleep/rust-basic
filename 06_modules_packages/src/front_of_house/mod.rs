// Front of house operations

pub mod hosting;

// Re-export hosting functions for convenience
pub use self::hosting::{add_to_waitlist, seat_at_table}; 