use user_domain::{GetUsersUseCaseImpl, DomainError};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create use case in a separate thread to avoid runtime nesting issues
    // GetUsersUseCaseImpl::new() creates its own runtime, so we need to call it outside of any tokio runtime
    let get_users_use_case = thread::spawn(|| {
        GetUsersUseCaseImpl::new()
    })
    .join()
    .map_err(|_| "Thread join error")?
    .map_err(|e: DomainError| -> Box<dyn std::error::Error> { Box::new(e) })?;
    
    // Now we can use tokio::main for the async operations
    tokio::runtime::Runtime::new()?.block_on(async {
        // Execute the use case to get all users
        println!("Executing GetUsersUseCase...");
        let users = get_users_use_case.execute().await
            .map_err(|e: DomainError| -> Box<dyn std::error::Error> { Box::new(e) })?;
        
        println!("\nRetrieved {} users", users.len());
        
        // Display first 3 users
        println!("\nFirst 3 users:");
        for user in users.iter().take(3) {
            println!(
                "  - {} {} (ID: {}, Email: {})",
                user.first_name, user.last_name, user.id, user.email
            );
        }
        
        // Execute the use case to get a specific user
        println!("\nExecuting GetUsersUseCase for user ID 1...");
        let user = get_users_use_case.execute_by_id(1).await
            .map_err(|e: DomainError| -> Box<dyn std::error::Error> { Box::new(e) })?;
        println!(
            "User: {} {} - {} - {}",
            user.first_name, user.last_name, user.email, user.phone
        );
        
        // Test business logic validation
        println!("\nTesting business logic validation (ID 0)...");
        match get_users_use_case.execute_by_id(0).await {
            Err(e) => {
                let error_display = get_users_use_case.to_error_display(e);
                println!("Expected error: {}: {}", error_display.title, error_display.subtitle);
            },
            Ok(_) => println!("Unexpected success"),
        }
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

