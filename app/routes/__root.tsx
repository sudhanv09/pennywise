import { Outlet, createRootRoute } from "@tanstack/solid-router";
import {
  Sidebar,
  SidebarProvider,
  SidebarTrigger,
  useSidebar,
} from "~/components/sidebar";
import styles from "./root.module.css";

export const Route = createRootRoute({
  head: () => ({
    meta: [
      {
        charset: "utf-8",
      },
      {
        name: "viewport",
        content: "width=device-width, initial-scale=1",
      },
      {
        title: "Pennywise",
      },
    ],
  }),
  component: RootComponent,
});

function RootComponent() {
  return (
    <SidebarProvider>
      <AppLayout />
    </SidebarProvider>
  );
}

function AppLayout() {
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
