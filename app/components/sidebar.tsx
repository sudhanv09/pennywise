import styles from "./sidebar.module.css";
import { Link, useRouter } from "@tanstack/solid-router";

export function Sidebar() {
  const path = useRouter();
  const currentPath = path.state.location.pathname;

  const links = [
    { label: "Dashboard", href: "/" },
    { label: "Transactions", href: "/transactions" },
    { label: "Calendar", href: "/calendar" },
    { label: "Activity Log", href: "/activity" },
  ];

  return (
    <aside class={styles.sidebar}>
      <div class={styles.sidebar__header}>
        <h1>Pennywise</h1>
      </div>
      <div class={styles.sidebar__content}>
        <div class={styles.sidebar__group}>
          <ul class={styles.sidebar__menu}>
            {links.map((link) => (
              <li>
                <Link
                  to={link.href}
                  class={`${styles.sidebar__menuitem}`}
                >
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
