# Network Crate

A reusable HTTP client crate for making API calls, designed to be shared across Android and iOS platforms.

## Features

- **Trait-based design** for dependency injection and testing
- **Async/await** support using tokio
- **JSON serialization/deserialization** support
- **Flexible request building** with builder pattern
- **Comprehensive error handling**
- **Configurable timeouts and headers**

## Usage

### Basic Usage

```rust
use network::{HTTPClient, HTTPClientImpl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = HTTPClientImpl::new()?;
    
    // Make a GET request
    let response = client.get("https://api.example.com/users").await?;
    
    println!("Status: {}", response.status_code);
    println!("Body: {}", response.text()?);
    
    Ok(())
}
```

### POST Request with JSON

```rust
use network::{HTTPClient, HTTPClientImpl};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HTTPClientImpl::new()?;
    
    let new_user = CreateUser {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    
    let response = client
        .post_json("https://api.example.com/users", &new_user)
        .await?;
    
    if response.is_success() {
        let user: User = response.json()?;
        println!("Created user: {:?}", user);
    }
    
    Ok(())
}
```

### Advanced Request Building

```rust
use network::{HTTPClient, HTTPClientImpl, Request};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HTTPClientImpl::new()?;
    
    // Build a custom request
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    
    let mut query_params = HashMap::new();
    query_params.insert("page".to_string(), "1".to_string());
    query_params.insert("limit".to_string(), "10".to_string());
    
    let request = Request::get("https://api.example.com/users")
        .headers(headers)
        .query_params(query_params)
        .timeout(30);
    
    let response = client.execute(request).await?;
    println!("Response: {}", response.text()?);
    
    Ok(())
}
```

### Client Configuration

```rust
use network::HTTPClientImpl;
use std::collections::HashMap;

// Create client with timeout
let client = HTTPClientImpl::with_timeout(30)?;

// Create client with default headers
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token123".to_string());
let client = HTTPClientImpl::with_headers(headers)?;

// Create client with both timeout and headers
let client = HTTPClientImpl::with_config(Some(30), Some(headers))?;
```

### Dependency Injection

The `HTTPClient` trait allows for easy dependency injection and testing:

```rust
use network::{HTTPClient, HTTPClientImpl};

struct UserRepository {
    client: Box<dyn HTTPClient>,
}

impl UserRepository {
    fn new(client: Box<dyn HTTPClient>) -> Self {
        Self { client }
    }
    
    async fn get_user(&self, id: u64) -> network::Result<String> {
        let url = format!("https://api.example.com/users/{}", id);
        let response = self.client.get(url).await?;
        Ok(response.text()?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Box::new(HTTPClientImpl::new()?);
    let repo = UserRepository::new(client);
    
    let user = repo.get_user(1).await?;
    println!("User: {}", user);
    
    Ok(())
}
```

## Error Handling

The crate provides comprehensive error types:

```rust
use network::{NetworkError, Result};

match client.get("https://api.example.com/users").await {
    Ok(response) => println!("Success: {}", response.status_code),
    Err(NetworkError::ConnectionError(msg)) => eprintln!("Connection failed: {}", msg),
    Err(NetworkError::TimeoutError(msg)) => eprintln!("Request timed out: {}", msg),
    Err(NetworkError::ResponseError(msg)) => eprintln!("Response error: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `async-trait` - Async trait support
- `url` - URL parsing






