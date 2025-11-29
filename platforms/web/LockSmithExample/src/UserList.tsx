import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { WasmGetUsersUseCase } from 'locksmith'
import './UserList.css'

// Type definitions for WASM bindings
type WasmUserDomainModel = {
  id: number
  first_name: string
  last_name: string
  phone: string
  full_name: string
  image_url: string
  age_display: string
  email_display: string
}

type WasmUserDomainResultModel = 
  | { type: 'loaded'; data: WasmUserDomainModel[] }
  | { type: 'empty'; data: { title: string; subtitle: string; button_title: string } }
  | { type: 'error'; display: { title: string; subtitle: string } }

type UserPresentationModel = {
  id: number
  displayName: string
  email: string
  ageDisplay: string
  phone: string
  imageUrl: string
}

type UserListState = {
  isLoading: boolean
  users: UserPresentationModel[]
  errorMessage: string | null
  showError: boolean
  emptyStateTitle: string | null
  emptyStateSubtitle: string | null
  emptyStateButtonTitle: string | null
  showEmptyState: boolean
}

function UserList() {
  const [state, setState] = useState<UserListState>({
    isLoading: false,
    users: [],
    errorMessage: null,
    showError: false,
    emptyStateTitle: null,
    emptyStateSubtitle: null,
    emptyStateButtonTitle: null,
    showEmptyState: false,
  })

  const [useCase] = useState(() => new WasmGetUsersUseCase())

  const loadUsers = async () => {
    setState(prev => ({ ...prev, isLoading: true, errorMessage: null, showError: false, showEmptyState: false }))

    try {
      const resultPromise = useCase.execute()
      const resultValue = await resultPromise
      
      // The result is a JavaScript object from wasm-bindgen
      // Convert to our type
      const result = resultValue as any as WasmUserDomainResultModel

      if (result.type === 'loaded') {
        const presentationUsers: UserPresentationModel[] = result.data.map((domainUser: WasmUserDomainModel) => ({
          id: domainUser.id,
          displayName: domainUser.full_name,
          email: domainUser.email_display, // Pre-formatted from Rust
          ageDisplay: domainUser.age_display, // Pre-formatted from Rust
          phone: domainUser.phone,
          imageUrl: domainUser.image_url,
        }))

        setState({
          isLoading: false,
          users: presentationUsers,
          errorMessage: null,
          showError: false,
          emptyStateTitle: null,
          emptyStateSubtitle: null,
          emptyStateButtonTitle: null,
          showEmptyState: false,
        })
      } else if (result.type === 'empty') {
        setState({
          isLoading: false,
          users: [],
          errorMessage: null,
          showError: false,
          emptyStateTitle: result.data.title,
          emptyStateSubtitle: result.data.subtitle,
          emptyStateButtonTitle: result.data.button_title,
          showEmptyState: true,
        })
      } else if (result.type === 'error') {
        setState({
          isLoading: false,
          users: [],
          errorMessage: `${result.display.title}: ${result.display.subtitle}`,
          showError: true,
          emptyStateTitle: null,
          emptyStateSubtitle: null,
          emptyStateButtonTitle: null,
          showEmptyState: false,
        })
      }
    } catch (error) {
      console.error('Failed to load users:', error)
      setState(prev => ({
        ...prev,
        isLoading: false,
        errorMessage: 'Failed to load users. Please try again.',
        showError: true,
      }))
    }
  }

  useEffect(() => {
    loadUsers()
  }, [])

  if (state.isLoading) {
    return (
      <div className="user-list-container">
        <div className="loading-container">
          <div className="spinner"></div>
          <p>Loading users...</p>
        </div>
      </div>
    )
  }

  if (state.showError) {
    return (
      <div className="user-list-container">
        <div className="error-container">
          <h2>Error</h2>
          <p>{state.errorMessage}</p>
          <button onClick={loadUsers} className="retry-button">
            Retry
          </button>
        </div>
      </div>
    )
  }

  if (state.showEmptyState) {
    return (
      <div className="user-list-container">
        <div className="empty-container">
          <h2>{state.emptyStateTitle}</h2>
          <p>{state.emptyStateSubtitle}</p>
          {state.emptyStateButtonTitle && (
            <button onClick={loadUsers} className="refresh-button">
              {state.emptyStateButtonTitle}
            </button>
          )}
        </div>
      </div>
    )
  }

  return (
    <div className="user-list-container">
      <div className="user-list-header">
        <Link to="/" className="back-link">← Back to Menu</Link>
        <h1>Users</h1>
        <button onClick={loadUsers} className="refresh-button">
          Refresh
        </button>
      </div>
      <div className="user-list">
        {state.users.map((user) => (
          <div key={user.id} className="user-card">
            <div className="user-avatar">
              <img
                src={user.imageUrl}
                alt={user.displayName}
                onError={(e) => {
                  const target = e.target as HTMLImageElement
                  target.style.display = 'none'
                  const fallback = target.nextElementSibling as HTMLElement
                  if (fallback) fallback.style.display = 'flex'
                }}
              />
              <div className="avatar-fallback" style={{ display: 'none' }}>
                {user.displayName.charAt(0).toUpperCase()}
              </div>
            </div>
            <div className="user-info">
              <h3 className="user-name">{user.displayName}</h3>
              <p className="user-email">{user.email}</p>
              <p className="user-age">{user.ageDisplay}</p>
              <p className="user-phone">{user.phone}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

export default UserList

