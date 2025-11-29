package com.locksmith.example.Views

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import com.locksmith.example.ViewModels.UserListViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

/**
 * User List Screen
 * Displays a list of users fetched from the use case
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun UserListScreen(navController: NavHostController) {
    val viewModel = remember { UserListViewModel() }
    val coroutineScope = rememberCoroutineScope()
    
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Users") },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { padding ->
        when {
            viewModel.isLoading && viewModel.users.isEmpty() -> {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(padding),
                    contentAlignment = Alignment.Center
                ) {
                    CircularProgressIndicator()
                }
            }
            viewModel.showError -> {
                ErrorView(
                    message = viewModel.errorMessage ?: "Unknown error",
                    onRetry = { viewModel.loadUsers(coroutineScope) },
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(padding)
                )
            }
            viewModel.users.isEmpty() -> {
                EmptyView(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(padding)
                )
            }
            else -> {
                UserListView(
                    users = viewModel.users,
                    onRefresh = { viewModel.loadUsers(coroutineScope) },
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(padding)
                )
            }
        }
    }
    
    LaunchedEffect(Unit) {
        if (viewModel.users.isEmpty()) {
            viewModel.loadUsers(coroutineScope)
        }
    }
}

