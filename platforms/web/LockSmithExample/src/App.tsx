import { useEffect } from 'react'
import { Routes, Route } from 'react-router-dom'
import { initialize_localization } from 'locksmith'
import DebugMenu from './DebugMenu'
import PasswordValidation from './PasswordValidation'
import UserList from './UserList'
import Benchmark from './Benchmark'
import './App.css'

function App() {
  // Initialize WASM localization on app start
  useEffect(() => {
    const initWasm = async () => {
      try {
        console.log('Initializing WASM...')
        await new Promise(resolve => setTimeout(resolve, 50))
        initialize_localization()
        console.log('WASM initialized successfully')
      } catch (error) {
        console.error('Failed to initialize WASM:', error)
      }
    }
    
    initWasm()
  }, [])

  return (
    <Routes>
      <Route path="/" element={<DebugMenu />} />
      <Route path="/password" element={<PasswordValidation />} />
      <Route path="/users" element={<UserList />} />
      <Route path="/benchmark" element={<Benchmark />} />
    </Routes>
  )
}

export default App

