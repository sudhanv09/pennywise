import styles from "./sidebar.module.css";
import { Link, useRouter } from "@tanstack/solid-router";
import { createContext, createSignal, useContext } from "solid-js";

type SidebarContextType = {
  isOpen: () => boolean;
  toggle: () => void;
  close: () => void;
  open: () => void;
};

const SidebarContext = createContext<SidebarContextType>();

export function SidebarProvider(props) {
  const [isOpen, setIsOpen] = createSignal(true);

  const context: SidebarContextType = {
    isOpen,
    toggle: () => setIsOpen((prev) => !prev),
    close: () => setIsOpen(false),
    open: () => setIsOpen(true),
  };

  return (
    <SidebarContext.Provider value={context}>
      {props.children}
    </SidebarContext.Provider>
  );
}

export function useSidebar() {
  const ctx = useContext(SidebarContext);
  if (!ctx) throw new Error("useSidebar must be used within a SidebarProvider");
  return ctx;
}

export function SidebarTrigger() {
  const { isOpen, toggle } = useSidebar();

  return (
    <button onClick={toggle} class={styles.sidebar__trigger}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="lucide lucide-menu-icon lucide-menu"
      >
        <path d="M4 12h16" />
        <path d="M4 18h16" />
        <path d="M4 6h16" />
      </svg>
    </button>
  );
}

export function Sidebar() {
  const { isOpen } = useSidebar();
  const path = useRouter();
  const currentPath = path.state.location.pathname;

  const links = [
    { label: "Dashboard", href: "/" },
    { label: "Transactions", href: "/transactions" },
    { label: "Calendar", href: "/calendar" },
    { label: "Activity Log", href: "/activity" },
  ];

  return (
    <aside class={`${styles.sidebar} ${!isOpen() ? styles.closed : ""}`}>
      <div class={styles.sidebar__header}>
        <h1>Pennywise</h1>
      </div>
      <div class={styles.sidebar__content}>
        <div class={styles.sidebar__group}>
          <ul class={styles.sidebar__menu}>
            {links.map((link) => (
              <li>
                <Link to={link.href} class={`${styles.sidebar__menuitem}`}>
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      </div>
      <div class={styles.sidebar__footer}>Footer</div>
    </aside>
  );
}
