import { SidebarTrigger } from "./sidebar";
import styles from "./nav.module.css";

export function Navbar() {
  return (
    <>
      <nav class={styles.nav}>
        <SidebarTrigger />
        <div class={styles.nav_group}>
          <button>+ Add transaction</button>
          <span>Profile</span>
        </div>
      </nav>
      <hr />
    </>
  );
}
