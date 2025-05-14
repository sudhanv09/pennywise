import { Outlet, createRootRoute } from "@tanstack/solid-router";
import { SidebarProvider } from "~/components/sidebar";

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
      <Outlet />
    </SidebarProvider>
  );
}
