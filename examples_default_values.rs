// Examples of Default Values in Rust

// ============================================
// 1. DEFAULT VALUES FOR STRUCT FIELDS (Class Parameters)
// ============================================

// Method 1: Using Default trait
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

// Implement Default trait to provide default values
impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Unknown".to_string(),
            age: 0,
            email: None,
        }
    }
}

// Usage:
// let person = Person::default();
// let person = Person { name: "John".to_string(), ..Default::default() };

// Method 2: Using Default::default() for individual fields
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            timeout: Some(30), // or None
        }
    }
}

// Method 3: Using builder pattern for more flexibility
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    timeout: u64,
    max_connections: usize,
}

impl ServerConfig {
    // Constructor with defaults
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            timeout: 30,
            max_connections: 100,
        }
    }

    // Builder methods for optional customization
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

// ============================================
// 2. DEFAULT VALUES FOR FUNCTION PARAMETERS
// ============================================

// Rust doesn't have default parameter values like Python or JavaScript.
// Instead, you use these patterns:

// Pattern 1: Using Option<T> for optional parameters
fn greet(name: &str, greeting: Option<&str>) {
    let greeting = greeting.unwrap_or("Hello");
    println!("{}, {}!", greeting, name);
}

// Usage:
// greet("Alice", None);              // Uses default "Hello"
// greet("Bob", Some("Hi"));          // Uses custom "Hi"

// Pattern 2: Using multiple function signatures (function overloading via traits)
// This is more idiomatic - provide a simple function and a detailed one
fn calculate_price(amount: f64) -> f64 {
    calculate_price_with_tax(amount, 0.1) // Default tax rate 10%
}

fn calculate_price_with_tax(amount: f64, tax_rate: f64) -> f64 {
    amount * (1.0 + tax_rate)
}

// Pattern 3: Using a config struct for many parameters
#[derive(Default)]
struct RequestOptions {
    timeout: Option<u64>,
    retries: u32,
    headers: Option<Vec<String>>,
}

impl RequestOptions {
    fn new() -> Self {
        Self::default()
    }
}

fn make_request(url: &str, options: RequestOptions) {
    let timeout = options.timeout.unwrap_or(30);
    let retries = options.retries;
    println!("Requesting {} with timeout: {}, retries: {}", url, timeout, retries);
}

// Pattern 4: Using impl Into<T> for flexible defaults
fn send_message(message: &str, priority: impl Into<Option<u8>>) {
    let priority = priority.into().unwrap_or(5); // Default priority 5
    println!("Sending message: {} (priority: {})", message, priority);
}

// ============================================
// 3. PRACTICAL EXAMPLE (Combining both)
// ============================================

#[derive(Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    max_connections: usize,
    timeout_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "admin".to_string(),
            password: "".to_string(),
            max_connections: 10,
            timeout_seconds: 30,
        }
    }
}

impl DatabaseConfig {
    // Function with default parameters using Option
    pub fn connect(
        host: Option<String>,
        port: Option<u16>,
        username: Option<String>,
    ) -> Self {
        let mut config = Self::default();
        
        if let Some(h) = host {
            config.host = h;
        }
        if let Some(p) = port {
            config.port = p;
        }
        if let Some(u) = username {
            config.username = u;
        }
        
        config
    }
}

// ============================================
// 4. MAIN FUNCTION WITH EXAMPLES
// ============================================

fn main() {
    println!("=== Struct Default Values ===");
    
    // Using Default trait
    let person1 = Person::default();
    println!("Default person: {:?}", person1);
    
    let person2 = Person {
        name: "Alice".to_string(),
        ..Default::default()
    };
    println!("Partial person: {:?}", person2);
    
    // Using builder pattern
    let server = ServerConfig::new()
        .with_host("example.com".to_string())
        .with_port(9000);
    println!("Server config: {:?}", server);
    
    println!("\n=== Function Default Parameters ===");
    
    // Using Option for optional parameters
    greet("Alice", None);
    greet("Bob", Some("Hi"));
    
    // Using multiple function signatures
    println!("Price: {}", calculate_price(100.0));
    println!("Price with custom tax: {}", calculate_price_with_tax(100.0, 0.15));
    
    // Using config struct
    let options = RequestOptions {
        timeout: Some(60),
        retries: 3,
        ..Default::default()
    };
    make_request("https://example.com", options);
    
    // Using impl Into<Option<T>>
    send_message("Hello", None::<u8>);
    send_message("Urgent", Some(9));
    
    // Database config example
    let db1 = DatabaseConfig::default();
    println!("\nDefault DB config: {:?}", db1);
    
    let db2 = DatabaseConfig::connect(
        Some("db.example.com".to_string()),
        Some(3306),
        None,
    );
    println!("Custom DB config: {:?}", db2);
}

