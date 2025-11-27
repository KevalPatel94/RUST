package com.locksmith.example

import android.app.Application
import uniffi.locksmith.initializeLocalization

class LockSmithApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
        initializeLocalization()
    }
}

