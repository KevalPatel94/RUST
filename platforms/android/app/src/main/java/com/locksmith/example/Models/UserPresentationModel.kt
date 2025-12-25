package com.locksmith.example.Models

/**
 * Presentation model for displaying user information in the UI
 */
data class UserPresentationModel(
    val id: ULong,
    val displayName: String,
    val email: String,
    val phone: String,
    val ageDisplay: String,
    val imageUrl: String
)






