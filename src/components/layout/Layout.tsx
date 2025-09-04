import { createSignal, type JSX } from 'solid-js'
import Sidebar from './Sidebar'
import styles from './Layout.module.css'

interface LayoutProps {
  children: JSX.Element
}

export default function Layout(props: LayoutProps): JSX.Element {
  const [sidebarOpen, setSidebarOpen] = createSignal(false)

  const toggleSidebar = () => {
    setSidebarOpen(!sidebarOpen())
  }

  const closeSidebar = () => {
    setSidebarOpen(false)
  }

  return (
    <div class={styles.layout}>
      {/* Mobile header with hamburger menu */}
      <header class={styles.mobileHeader}>
        <button
          class={styles.hamburgerButton}
          onClick={toggleSidebar}
          aria-label="Open navigation menu"
          aria-expanded={sidebarOpen()}
        >
          <span class={styles.hamburgerLine}></span>
          <span class={styles.hamburgerLine}></span>
          <span class={styles.hamburgerLine}></span>
        </button>
        <h1 class={styles.mobileTitle}>💰 Penny</h1>
      </header>

      {/* Sidebar */}
      <Sidebar isOpen={sidebarOpen()} onToggle={closeSidebar} />

      {/* Main content area */}
      <main class={styles.main}>
        <div class={styles.content}>
          {props.children}
        </div>
      </main>
    </div>
  )
}