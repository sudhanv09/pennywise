import { SidebarTrigger } from "./sidebar";
import styles from "./nav.module.css";
import TransactionModal from "./transaction-modal";

export function Navbar() {
  return (
    <>
      <nav class={styles.nav}>
        <SidebarTrigger />
        <div class={styles.nav_group}>
          <TransactionModal />
          <span>Profile</span>
        </div>
      </nav>
      <hr />
    </>
  );
}
