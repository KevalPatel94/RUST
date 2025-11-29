import { Link } from 'react-router-dom'
import './DebugMenu.css'

type MenuItem = {
  title: string
  path: string
}

const MENU_ITEMS: MenuItem[] = [
  { title: 'LockSmith', path: '/password' },
  { title: 'User List', path: '/users' },
  { title: 'Benchmark', path: '/benchmark' },
]

function DebugMenu() {
  return (
    <div className="debug-menu-container">
      <header className="debug-menu-header">
        <h1>LockSmith Debug Menu</h1>
        <p>Select a feature to test</p>
      </header>
      <nav className="debug-menu-nav">
        {MENU_ITEMS.map((item) => (
          <Link key={item.path} to={item.path} className="debug-menu-item">
            <span className="menu-item-title">{item.title}</span>
            <span className="menu-item-arrow">→</span>
          </Link>
        ))}
      </nav>
    </div>
  )
}

export default DebugMenu

