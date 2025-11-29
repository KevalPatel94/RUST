package com.locksmith.example.ViewModels

import androidx.compose.runtime.Stable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import uniffi.user_domain.GetUsersUseCaseImpl
import uniffi.user_domain.UserDomainResultModel
import uniffi.domain_common.ErrorDisplay
import com.locksmith.example.Models.UserPresentationModel
import uniffi.user_domain.GetUsersUseCaseImplInterface

typealias GetUsersUseCase = GetUsersUseCaseImplInterface
/**
 * ViewModel for User List Screen
 * Manages state and business logic for displaying a list of users
 */
@Stable
class UserListViewModel {
    var users by mutableStateOf<List<UserPresentationModel>>(emptyList())
        private set
    
    var isLoading by mutableStateOf(false)
        private set
    
    var showError by mutableStateOf(false)
        private set
    
    var errorMessage by mutableStateOf<String?>(null)
        private set
    
    private val useCase: GetUsersUseCase = try {
        GetUsersUseCaseImpl()
    } catch (e: Exception) {
        throw RuntimeException("Failed to initialize GetUsersUseCaseImpl", e)
    }
    
    /**
     * Load users from the use case
     * @param coroutineScope The coroutine scope to launch the async operation
     */
    fun loadUsers(coroutineScope: CoroutineScope) {
        isLoading = true
        errorMessage = null
        showError = false
        
        // Use Kotlin coroutines to call async Rust function
        coroutineScope.launch {
            val result = useCase.execute()
            
            when (result) {
                is UserDomainResultModel.Loaded -> {
                    val presentationUsers = result.data.map { domainUser ->
                        UserPresentationModel(
                            id = domainUser.id,
                            displayName = domainUser.fullName,
                            email = domainUser.emailDisplay,
                            phone = domainUser.phone,
                            ageDisplay = domainUser.ageDisplay,
                            imageUrl = domainUser.imageUrl
                        )
                    }
                    users = presentationUsers
                    isLoading = false
                    showError = false
                }
                is UserDomainResultModel.Empty -> {
                    // Empty case has EpmtyDataModel with title/subtitle/button_title
                    users = emptyList()
                    isLoading = false
                    showError = false
                    // Can use result.data.title, result.data.subtitle, result.data.buttonTitle
                }
                is UserDomainResultModel.Error -> {
                    isLoading = false
                    errorMessage = formatError(result.display)
                    showError = true
                }
            }
        }
    }
    
    private fun formatError(errorDisplay: ErrorDisplay): String {
        return "${errorDisplay.title}: ${errorDisplay.subtitle}"
    }
}

