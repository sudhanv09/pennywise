import { type JSX } from 'solid-js'
import { Link, useLocation } from '@tanstack/solid-router'
import styles from './Sidebar.module.css'

interface SidebarProps {
  isOpen: boolean
  onToggle: () => void
}

interface NavItem {
  path: string
  label: string
  icon: string
}

const navItems: NavItem[] = [
  { path: '/dashboard', label: 'Dashboard', icon: '📊' },
  { path: '/add-transaction', label: 'Add Transaction', icon: '➕' },
  { path: '/transactions', label: 'Transactions', icon: '📋' },
]

export default function Sidebar(props: SidebarProps): JSX.Element {
  const location = useLocation({select: (location) => location.pathname})

  const isActiveRoute = (path: string): boolean => {
    const currentPath = location()
    // Handle root path redirect to dashboard
    if (currentPath === '/' && path === '/dashboard') {
      return true
    }
    return currentPath === path
  }

  return (
    <>
      {/* Mobile overlay */}
      <div 
        class={`${styles.overlay} ${props.isOpen ? styles.overlayOpen : ''}`}
        onClick={props.onToggle}
        aria-hidden="true"
      />
      
      {/* Sidebar */}
      <aside 
        class={`${styles.sidebar} ${props.isOpen ? styles.sidebarOpen : ''}`}
        aria-label="Main navigation"
      >
        <div class={styles.sidebarHeader}>
          <h2 class={styles.logo}>💰 Penny</h2>
          <button
            class={styles.closeButton}
            onClick={props.onToggle}
            aria-label="Close navigation"
          >
            ✕
          </button>
        </div>

        <nav class={styles.nav} role="navigation">
          <ul class={styles.navList}>
            {navItems.map((item) => (
              <li class={styles.navItem}>
                <Link
                  to={item.path}
                  class={`${styles.navLink} ${isActiveRoute(item.path) ? styles.navLinkActive : ''}`}
                  onClick={props.onToggle}
                  aria-current={isActiveRoute(item.path) ? 'page' : undefined}
                >
                  <span class={styles.navIcon} aria-hidden="true">
                    {item.icon}
                  </span>
                  <span class={styles.navLabel}>{item.label}</span>
                </Link>
              </li>
            ))}
          </ul>
        </nav>

        <div class={styles.sidebarFooter}>
          <p class={styles.footerText}>Expense Tracker v1.0</p>
        </div>
      </aside>
    </>
  )
}