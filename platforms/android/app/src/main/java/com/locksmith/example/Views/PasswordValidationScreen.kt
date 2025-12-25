package com.locksmith.example.Views

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.navigation.NavHostController
import uniffi.locksmith.*

/**
 * Locale options for the password validation screen
 */
private enum class LocaleOption(val code: String, val title: String) {
    ENGLISH("en", "English"),
    SPANISH("es", "Español"),
    FRENCH("fr", "Français")
}

/**
 * Sample passwords for testing validation
 */
private enum class SamplePassword(
    val localizationKey: String,
    val value: String
) {
    TOO_SHORT("password-sample-too-short", "Ab1!"),
    TOO_LONG("password-sample-too-long", "Abcdefghijklmnopqrstu1!"),
    NO_UPPERCASE("password-sample-no-uppercase", "abc1!abc"),
    NO_LOWERCASE("password-sample-no-lowercase", "ABC1!ABC"),
    NO_NUMBER("password-sample-no-number", "Abc!Abcd"),
    NO_SYMBOL("password-sample-no-symbol", "Abc1Abcd"),
    VALID("password-sample-valid", "Abc1!abc")
}

/**
 * Password Validation Screen
 * Allows users to test password validation with different locales
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PasswordValidationScreen(
    navController: NavHostController,
    validator: PasswordValidator
) {
    var password by remember { mutableStateOf("") }
    var isPasswordVisible by remember { mutableStateOf(false) }
    var selectedLocale by remember { mutableStateOf(LocaleOption.ENGLISH) }
    
    // Initialize locale on first composition
    LaunchedEffect(selectedLocale) {
        setAppLocale(selectedLocale.code)
    }
    
    val validationMessage = remember(password) {
        if (password.isEmpty()) {
            ""
        } else {
            validator.validateWithMessage(password)
        }
    }
    
    val isValid = remember(validationMessage) {
        validationMessage == getLocalizedText("password-valid")
    }
    
    val validationColor = remember(validationMessage, isValid) {
        when {
            password.isEmpty() -> Color(0xFFFF0000) // Red
            isValid -> Color(0xFF4CAF50) // Green
            else -> Color(0xFFFF0000) // Red
        }
    }
    
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(getRustDemoTitle("Android")) },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(rememberScrollState())
                .padding(padding)
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // Locale Label
            Text(
                text = getLocalizedText("locale-picker-label"),
                fontSize = 14.sp
            )
            
            // Locale Selector
            Row(
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                modifier = Modifier.padding(vertical = 8.dp)
            ) {
                LocaleOption.values().forEach { option ->
                    FilterChip(
                        selected = selectedLocale == option,
                        onClick = {
                            selectedLocale = option
                            setAppLocale(option.code)
                        },
                        label = { Text(option.title) }
                    )
                }
            }
            
            // Password Field with Show/Hide Button
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                OutlinedTextField(
                    value = password,
                    onValueChange = { password = it },
                    modifier = Modifier.weight(1f),
                    label = { Text(getLocalizedText("password-input-placeholder")) },
                    visualTransformation = if (isPasswordVisible) {
                        androidx.compose.ui.text.input.VisualTransformation.None
                    } else {
                        PasswordVisualTransformation()
                    },
                    singleLine = true
                )
                
                Button(
                    onClick = { isPasswordVisible = !isPasswordVisible },
                    colors = ButtonDefaults.buttonColors(
                        containerColor = Color(0xFF2196F3)
                    )
                ) {
                    Text(
                        text = getLocalizedText(
                            if (isPasswordVisible) "password-toggle-hide" else "password-toggle-show"
                        )
                    )
                }
            }
            
            // Validation Message
            if (validationMessage.isNotEmpty()) {
                Text(
                    text = validationMessage,
                    color = validationColor,
                    fontSize = 16.sp,
                    modifier = Modifier.fillMaxWidth()
                )
            }
            
            // Instructions
            Text(
                text = getLocalizedText("password-instructions"),
                fontSize = 12.sp,
                color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f),
                modifier = Modifier.fillMaxWidth()
            )
            
            // Sample Passwords Label
            Text(
                text = getLocalizedText("password-sample-header"),
                fontSize = 16.sp,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.fillMaxWidth()
            )
            
            // Sample Password Buttons
            SamplePassword.values().forEach { sample ->
                Button(
                    onClick = {
                        password = sample.value
                    },
                    modifier = Modifier
                        .fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = Color(0xFF2196F3)
                    )
                ) {
                    Text(
                        text = "•${getLocalizedText(sample.localizationKey)}",
                        modifier = Modifier.fillMaxWidth(),
                        textAlign = TextAlign.Left
                    )
                }
            }
        }
    }
}






