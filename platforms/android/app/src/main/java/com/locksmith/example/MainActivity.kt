package com.locksmith.example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Stable
import androidx.compose.runtime.rememberCoroutineScope
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.draw.clip
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import coil.compose.AsyncImage
import coil.compose.SubcomposeAsyncImage
import coil.compose.SubcomposeAsyncImageContent
import coil.request.ImageRequest as CoilImageRequest
import uniffi.locksmith.*
import uniffi.locksmith.getLocalizedText
import uniffi.locksmith.getRustDemoTitle
import uniffi.locksmith.setAppLocale
import uniffi.user_domain.GetUsersUseCaseImpl
import uniffi.user_domain.UserDomainModel
import uniffi.user_domain.UserDomainResultModel
import uniffi.domain_common.ErrorDisplay

class MainActivity : ComponentActivity() {
    
    private enum class LocaleOption(val code: String, val title: String) {
        ENGLISH("en", "English"),
        SPANISH("es", "Español"),
        FRENCH("fr", "Français")
    }

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
    
    private enum class DebugMenuItem(val title: String, val route: String) {
        LOCKSMITH("LockSmith", "password_screen"),
        USER_LIST("User List", "user_list_screen")
    }
    
    private val validator = PasswordValidator()
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LockSmithTheme {
                Navigation()
            }
        }
    }
    
    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    fun Navigation() {
        val navController = rememberNavController()
        
        NavHost(
            navController = navController,
            startDestination = "debug_menu"
        ) {
            composable("debug_menu") {
                DebugMenuScreen(navController = navController)
            }
            composable("password_screen") {
                PasswordValidationScreen(navController = navController)
            }
            composable("user_list_screen") {
                UserListScreen(navController = navController)
            }
        }
    }
    
    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    fun DebugMenuScreen(navController: NavHostController) {
        Scaffold(
            topBar = {
                TopAppBar(
                    title = { Text("Debug Menu") }
                )
            }
        ) { padding ->
            LazyColumn(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(DebugMenuItem.values().toList()) { item ->
                    Card(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable {
                                navController.navigate(item.route)
                            },
                        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
                    ) {
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(16.dp),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(
                                text = item.title,
                                fontSize = 16.sp,
                                fontWeight = FontWeight.Medium
                            )
                            Text(
                                text = "→",
                                fontSize = 16.sp,
                                color = MaterialTheme.colorScheme.primary
                            )
                        }
                    }
                }
            }
        }
    }
    
    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    fun PasswordValidationScreen(navController: NavHostController) {
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
                            androidx.compose.ui.text.input.PasswordVisualTransformation()
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
    
    @Composable
    fun LockSmithTheme(content: @Composable () -> Unit) {
        MaterialTheme(
            colorScheme = MaterialTheme.colorScheme,
            content = content
        )
    }
    
    // MARK: - User List Screen
    
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
    
    @Composable
    private fun UserListView(
        users: List<UserPresentationModel>,
        onRefresh: () -> Unit,
        modifier: Modifier = Modifier
    ) {
        LazyColumn(
            modifier = modifier,
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            items(users) { user ->
                UserRowView(user = user)
            }
        }
    }
    
    @Composable
    private fun UserRowView(user: UserPresentationModel) {
        Card(
            modifier = Modifier.fillMaxWidth(),
            elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                // User avatar with lazy loading
                SubcomposeAsyncImage(
                    model = CoilImageRequest.Builder(LocalContext.current)
                        .data(user.imageUrl)
                        .crossfade(true)
                        .build(),
                    contentDescription = "User avatar for ${user.displayName}",
                    modifier = Modifier
                        .size(50.dp)
                        .clip(androidx.compose.foundation.shape.CircleShape),
                    // FIXED: Remove `state ->` and use implicit receiver `this`
                    content = {
                        val currentPainter = painter
                        when (currentPainter.state) {
                            is coil.compose.AsyncImagePainter.State.Loading,
                            is coil.compose.AsyncImagePainter.State.Empty -> {
                                // Loading State
                                Box(
                                    modifier = Modifier
                                        .size(50.dp)
                                        .background(
                                            Color.Gray.copy(alpha = 0.3f),
                                            shape = androidx.compose.foundation.shape.CircleShape
                                        ),
                                    contentAlignment = Alignment.Center
                                ) {
                                    CircularProgressIndicator(
                                        modifier = Modifier.size(24.dp),
                                        strokeWidth = 2.dp,
                                        color = Color.Gray
                                    )
                                }
                            }
                            is coil.compose.AsyncImagePainter.State.Error -> {
                                // Error / Fallback state
                                Box(
                                    modifier = Modifier
                                        .size(50.dp)
                                        .background(
                                            Color.Gray.copy(alpha = 0.3f),
                                            shape = androidx.compose.foundation.shape.CircleShape
                                        ),
                                    contentAlignment = Alignment.Center
                                ) {
                                    Text(
                                        text = user.displayName.firstOrNull()?.toString() ?: "?",
                                        fontSize = 20.sp,
                                        fontWeight = FontWeight.Bold,
                                        color = Color.Gray
                                    )
                                }
                            }
                            is coil.compose.AsyncImagePainter.State.Success -> {
                                // Success State
                                SubcomposeAsyncImageContent()
                            }
                        }
                    }
                )
                
                // User info
                Column(
                    modifier = Modifier.weight(1f),
                    verticalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    Text(
                        text = user.displayName,
                        fontSize = 16.sp,
                        fontWeight = FontWeight.Medium
                    )
                    Text(
                        text = user.email,
                        fontSize = 14.sp,
                        color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
                    )
                    Text(
                        text = user.ageDisplay,
                        fontSize = 12.sp,
                        color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
                    )
                }
            }
        }
    }
    
    @Composable
    private fun EmptyView(modifier: Modifier = Modifier) {
        Column(
            modifier = modifier,
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            Text(
                text = "No users found",
                fontSize = 18.sp,
                color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
            )
        }
    }
    
    @Composable
    private fun ErrorView(
        message: String,
        onRetry: () -> Unit,
        modifier: Modifier = Modifier
    ) {
        Column(
            modifier = modifier,
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
            content = {
                Text(
                    text = message,
                    fontSize = 16.sp,
                    color = MaterialTheme.colorScheme.error,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.padding(16.dp)
                )
                Button(onClick = onRetry) {
                    Text("Retry")
                }
            }
        )
    }
}

// MARK: - User List ViewModel

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
    
    private val useCase: GetUsersUseCaseImpl = try {
        GetUsersUseCaseImpl()
    } catch (e: Exception) {
        throw RuntimeException("Failed to initialize GetUsersUseCaseImpl", e)
    }
    
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

// MARK: - Presentation Model

data class UserPresentationModel(
    val id: ULong,
    val displayName: String,
    val email: String,
    val phone: String,
    val ageDisplay: String,
    val imageUrl: String
)
