package com.locksmith.example.Views

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.SubcomposeAsyncImage
import coil.compose.SubcomposeAsyncImageContent
import coil.request.ImageRequest as CoilImageRequest
import com.locksmith.example.Models.UserPresentationModel

/**
 * User List View Component
 * Displays a scrollable list of users
 */
@Composable
fun UserListView(
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

/**
 * User Row View Component
 * Displays a single user in the list
 */
@Composable
fun UserRowView(user: UserPresentationModel) {
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
                    .clip(CircleShape),
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
                                        shape = CircleShape
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
                                        shape = CircleShape
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

/**
 * Empty View Component
 * Displays when no users are available
 */
@Composable
fun EmptyView(modifier: Modifier = Modifier) {
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

/**
 * Error View Component
 * Displays error message with retry button
 */
@Composable
fun ErrorView(
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

