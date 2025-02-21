use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};

// Book category enum
#[derive(Debug, Clone, PartialEq)]
enum BookCategory {
    Fiction,
    NonFiction,
    Reference,
    Children,
    TextBook,
}

// Membership type enum
#[derive(Debug, Clone, PartialEq)]
enum MembershipType {
    Standard,
    Premium,
    Student,
}

// Loan status enum
#[derive(Debug, Clone, PartialEq)]
enum LoanStatus {
    Active,
    Overdue,
    Returned,
}

// Book struct
#[derive(Debug, Clone)]
struct Book {
    id: String,
    title: String,
    author: String,
    category: BookCategory,
    available: bool,
}

// Member struct
#[derive(Debug, Clone)]
struct Member {
    id: String,
    name: String,
    membership: MembershipType,
    active_loans: Vec<String>, // Book IDs
}

// Loan struct
#[derive(Debug, Clone)]
struct Loan {
    book_id: String,
    member_id: String,
    checkout_date: SystemTime,
    due_date: SystemTime,
    status: LoanStatus,
}

// Library management system
struct Library {
    books: HashMap<String, Book>,
    members: HashMap<String, Member>,
    loans: HashMap<String, Loan>,
}

impl Library {
    fn new() -> Self {
        Library {
            books: HashMap::new(),
            members: HashMap::new(),
            loans: HashMap::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.insert(book.id.clone(), book);
    }

    fn add_member(&mut self, member: Member) {
        self.members.insert(member.id.clone(), member);
    }

    fn checkout_book(&mut self, book_id: &str, member_id: &str) -> Result<(), String> {
        // Verify book exists and is available
        let book = self.books.get_mut(book_id).ok_or("Book not found")?;
        if !book.available {
            return Err("Book is not available".to_string());
        }

        // Verify member exists and can borrow
        let member = self.members.get_mut(member_id).ok_or("Member not found")?;
        let max_loans = match member.membership {
            MembershipType::Standard => 3,
            MembershipType::Premium => 5,
            MembershipType::Student => 4,
        };

        if member.active_loans.len() >= max_loans {
            return Err("Member has reached maximum loans".to_string());
        }

        // Create loan
        let loan = Loan {
            book_id: book_id.to_string(),
            member_id: member_id.to_string(),
            checkout_date: SystemTime::now(),
            due_date: SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 14), // 14 days
            status: LoanStatus::Active,
        };

        // Update book and member
        book.available = false;
        member.active_loans.push(book_id.to_string());

        // Store loan
        self.loans.insert(format!("{}-{}", book_id, member_id), loan);

        Ok(())
    }

    fn return_book(&mut self, book_id: &str, member_id: &str) -> Result<(), String> {
        let loan_id = format!("{}-{}", book_id, member_id);
        
        // Verify loan exists
        let loan = self.loans.get_mut(&loan_id).ok_or("Loan not found")?;
        if loan.status == LoanStatus::Returned {
            return Err("Book already returned".to_string());
        }

        // Update book availability
        let book = self.books.get_mut(book_id).ok_or("Book not found")?;
        book.available = true;

        // Update member's active loans
        let member = self.members.get_mut(member_id).ok_or("Member not found")?;
        member.active_loans.retain(|id| id != book_id);

        // Update loan status
        loan.status = LoanStatus::Returned;

        Ok(())
    }

    fn get_member_loans(&self, member_id: &str) -> Vec<(&Book, &Loan)> {
        self.loans
            .iter()
            .filter(|(_, loan)| loan.member_id == member_id && loan.status == LoanStatus::Active)
            .filter_map(|(_, loan)| {
                self.books
                    .get(&loan.book_id)
                    .map(|book| (book, loan))
            })
            .collect()
    }

    fn check_overdue_loans(&mut self) {
        let now = SystemTime::now();
        for loan in self.loans.values_mut() {
            if loan.status == LoanStatus::Active && loan.due_date < now {
                loan.status = LoanStatus::Overdue;
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut library = Library::new();

    // Add some sample books
    library.add_book(Book {
        id: "B001".to_string(),
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        category: BookCategory::Fiction,
        available: true,
    });

    library.add_book(Book {
        id: "B002".to_string(),
        title: "Introduction to Algorithms".to_string(),
        author: "Thomas H. Cormen".to_string(),
        category: BookCategory::TextBook,
        available: true,
    });

    // Add a sample member
    library.add_member(Member {
        id: "M001".to_string(),
        name: "John Doe".to_string(),
        membership: MembershipType::Standard,
        active_loans: Vec::new(),
    });

    println!("Library Management System");
    println!("------------------------");

    loop {
        println!("\nOptions:");
        println!("1. Checkout book");
        println!("2. Return book");
        println!("3. View member loans");
        println!("4. Check overdue loans");
        println!("5. Quit");

        let choice = get_user_input("\nSelect option (1-5): ");

        match choice.as_str() {
            "1" => {
                let book_id = get_user_input("Enter book ID: ");
                let member_id = get_user_input("Enter member ID: ");
                
                match library.checkout_book(&book_id, &member_id) {
                    Ok(_) => println!("Book checked out successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                let book_id = get_user_input("Enter book ID: ");
                let member_id = get_user_input("Enter member ID: ");
                
                match library.return_book(&book_id, &member_id) {
                    Ok(_) => println!("Book returned successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                let member_id = get_user_input("Enter member ID: ");
                let loans = library.get_member_loans(&member_id);
                
                if loans.is_empty() {
                    println!("No active loans found.");
                } else {
                    println!("\nActive loans:");
                    for (book, loan) in loans {
                        println!("- {} by {} (Due: {:?})",
                            book.title, book.author,
                            loan.due_date.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() / (24 * 60 * 60));
                    }
                }
            }
            "4" => {
                library.check_overdue_loans();
                println!("Checked for overdue loans.");
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please select 1-5."),
        }
    }
} 