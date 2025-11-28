use user_data::{UserRepository, UserRepositoryImpl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Create user repository with HTTP client dependency
    let user_repo = UserRepositoryImpl::new()?;
    
    // Fetch all users
    println!("Fetching all users from https://dummyjson.com/users...");
    let users_response = user_repo.get_users().await?;
    
    println!("\nTotal users: {}", users_response.total);
    println!("Fetched {} users (limit: {})", users_response.users.len(), users_response.limit);
    
    // Display first few users
    println!("\nFirst 3 users:");
    for user in users_response.users.iter().take(3) {
        println!(
            "  - {} {} (ID: {}, Email: {})",
            user.first_name, user.last_name, user.id, user.email
        );
    }
    
    // Fetch a specific user by ID
    println!("\nFetching user with ID 1...");
    let user = user_repo.get_user_by_id(1).await?;
    println!(
        "User: {} {} - {} - {}",
        user.first_name, user.last_name, user.email, user.phone
    );
    
    Ok(())
}

