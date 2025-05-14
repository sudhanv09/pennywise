import { createFileRoute, Outlet } from "@tanstack/solid-router";
import styles from "./layout.module.css";

import { useSidebar, Sidebar, SidebarTrigger } from "~/components/sidebar";

export const Route = createFileRoute("/app")({
  component: RouteComponent,
});

function RouteComponent() {
  const { isOpen } = useSidebar();
  return (
    <div
      class={`${styles.root} ${
        isOpen() ? styles.sidebarOpen : styles.sidebarClosed
      }`}
    >
      <Sidebar />
      <div class={styles.app}>
        <SidebarTrigger />
        <Outlet />
      </div>
    </div>
  );
}
