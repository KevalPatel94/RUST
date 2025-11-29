package com.locksmith.example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.locksmith.example.Views.*
import uniffi.locksmith.PasswordValidator

/**
 * Main Activity
 * Entry point of the application, sets up navigation
 */
class MainActivity : ComponentActivity() {
    
    private val validator = PasswordValidator()
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LockSmithTheme {
                Navigation(validator = validator)
            }
        }
    }
    
    @Composable
    fun Navigation(validator: PasswordValidator) {
        val navController = rememberNavController()
        
        NavHost(
            navController = navController,
            startDestination = "debug_menu"
        ) {
            composable("debug_menu") {
                DebugMenuScreen(navController = navController)
            }
            composable("password_screen") {
                PasswordValidationScreen(
                    navController = navController,
                    validator = validator
                )
            }
            composable("user_list_screen") {
                UserListScreen(navController = navController)
            }
        }
    }
    
    @Composable
    fun LockSmithTheme(content: @Composable () -> Unit) {
        MaterialTheme(
            colorScheme = MaterialTheme.colorScheme,
            content = content
        )
    }
}
