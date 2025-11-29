use user_domain::{GetUsersUseCaseImpl, UserDomainResultModel};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create use case in a separate thread to avoid runtime nesting issues
    // GetUsersUseCaseImpl::new() creates its own runtime, so we need to call it outside of any tokio runtime
    let get_users_use_case = thread::spawn(|| {
        GetUsersUseCaseImpl::new()
    })
    .join()
    .map_err(|_| "Thread join error")?;
    
    // Now we can use tokio::main for the async operations
    tokio::runtime::Runtime::new()?.block_on(async {
        // Execute the use case to get all users
        println!("Executing GetUsersUseCase...");
        let result = get_users_use_case.execute().await;
        
        match result {
            UserDomainResultModel::Loaded { data: users } => {
                println!("\n✅ Successfully retrieved {} users", users.len());
                
                // Display first 3 users
                println!("\nFirst 3 users:");
                for user in users.iter().take(3) {
                    println!(
                        "  - {} (ID: {}, {}, {})",
                        user.full_name, user.id, user.email_display, user.age_display
                    );
                }
            }
            UserDomainResultModel::Empty { data: empty_data } => {
                println!("\n⚠️  Empty state:");
                println!("  Title: {}", empty_data.title);
                println!("  Subtitle: {}", empty_data.subtitle);
                println!("  Button: {}", empty_data.button_title);
            }
            UserDomainResultModel::Error { display } => {
                println!("\n❌ Error occurred:");
                println!("  {}: {}", display.title, display.subtitle);
            }
        }
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}
