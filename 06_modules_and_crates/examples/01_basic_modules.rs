// Example: Basic Module Organization
// This example demonstrates basic module organization, visibility rules, and module usage

// Define a module for user-related functionality
mod user {
    // Public struct that can be used outside the module
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub email: String,
        // Private field - only accessible within the module
        password_hash: String,
    }
    
    impl User {
        // Public constructor
        pub fn new(username: &str, email: &str, password: &str) -> Self {
            User {
                username: username.to_string(),
                email: email.to_string(),
                password_hash: hash_password(password),
            }
        }
        
        // Public method
        pub fn display_info(&self) {
            println!("User: {}", self.username);
            println!("Email: {}", self.email);
            // Can access private fields here
            println!("Password hash: {}", self.password_hash);
        }
    }
    
    // Private function - only accessible within the module
    fn hash_password(password: &str) -> String {
        // Simple hash simulation
        format!("hashed_{}", password)
    }
    
    // Submodule for authentication
    pub mod auth {
        // Public function in a nested module
        pub fn validate_password(password: &str) -> bool {
            // Access parent module's function
            let hashed = super::hash_password(password);
            hashed.starts_with("hashed_")
        }
        
        // Private function
        fn generate_token() -> String {
            "token123".to_string()
        }
        
        // Public function that uses private functionality
        pub fn login(password: &str) -> Option<String> {
            if validate_password(password) {
                Some(generate_token())
            } else {
                None
            }
        }
    }
}

// Another module demonstrating module relationships
mod permissions {
    use super::user::User;
    
    pub enum Role {
        Admin,
        Regular,
        Guest,
    }
    
    pub fn check_access(user: &User, role: Role) -> bool {
        match role {
            Role::Admin => user.username == "admin",
            Role::Regular => true,
            Role::Guest => true,
        }
    }
}

fn main() {
    // Using the public components of our modules
    let user = user::User::new(
        "john_doe",
        "john@example.com",
        "secret123"
    );
    
    // Using public methods
    user.display_info();
    
    // Using nested module functionality
    if user::auth::validate_password("secret123") {
        println!("Password is valid!");
    }
    
    if let Some(token) = user::auth::login("secret123") {
        println!("Login successful! Token: {}", token);
    }
    
    // Using another module's functionality
    let is_admin = permissions::check_access(
        &user,
        permissions::Role::Admin
    );
    println!("Is admin? {}", is_admin);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = user::User::new(
            "test_user",
            "test@example.com",
            "password123"
        );
        assert_eq!(user.username, "test_user");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[test]
    fn test_auth_validation() {
        assert!(user::auth::validate_password("any_password"));
    }
    
    #[test]
    fn test_permissions() {
        let regular_user = user::User::new(
            "regular",
            "regular@example.com",
            "pass123"
        );
        let admin_user = user::User::new(
            "admin",
            "admin@example.com",
            "pass123"
        );
        
        assert!(!permissions::check_access(
            &regular_user,
            permissions::Role::Admin
        ));
        assert!(permissions::check_access(
            &admin_user,
            permissions::Role::Admin
        ));
    }
} 