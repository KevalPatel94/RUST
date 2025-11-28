use crate::domain_error::{DomainError, ErrorDisplay};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Base UseCase that handles common concerns for all UseCases
/// - Runtime management for async operations
/// - Error conversion
/// - Common async execution patterns
/// 
/// This is shared across all domain crates (user_domain, product_domain, etc.)
pub struct BaseUseCase {
    runtime: Arc<Runtime>,
}

impl BaseUseCase {
    /// Create a new BaseUseCase with a tokio runtime
    pub fn new() -> Result<Self, DomainError> {
        let runtime = Runtime::new()
            .map_err(|_| DomainError::Error {
                display: ErrorDisplay::generic(),
            })?;
        
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// Execute an async operation on the internal runtime
    /// This handles spawning tasks and error conversion automatically
    pub async fn execute_async<F, T>(&self, f: F) -> Result<T, DomainError>
    where
        F: std::future::Future<Output = Result<T, ErrorDisplay>> + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.runtime.handle().clone();
        
        handle.spawn(f)
            .await
            .map_err(|_| DomainError::Error {
                display: ErrorDisplay::generic(),
            })?
            .map_err(|e| DomainError::from(e))
    }

    /// Execute an async operation that returns ErrorDisplay directly
    /// Useful when the operation already returns ErrorDisplay
    pub async fn execute_with_error_display<F, T>(&self, f: F) -> Result<T, DomainError>
    where
        F: std::future::Future<Output = Result<T, ErrorDisplay>> + Send + 'static,
        T: Send + 'static,
    {
        self.execute_async(f).await
    }

    /// Get the runtime handle for advanced use cases
    /// Use this when you need to block on async operations during initialization
    pub fn runtime_handle(&self) -> tokio::runtime::Handle {
        self.runtime.handle().clone()
    }

    /// Convert ErrorDisplay to DomainError
    pub fn to_domain_error(&self, error: ErrorDisplay) -> DomainError {
        DomainError::from(error)
    }

    /// Get a reference to the internal runtime
    /// This is useful for advanced scenarios where you need direct runtime access
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
}

