import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import { SidebarProvider, SidebarInset } from "~/components/ui/sidebar";
import { AppSidebar } from "~/components/app/appsidebar";
import AppNav from "~/components/app/nav";
import "./app.css";

export default function App() {
  return (
    <Router
      root={(props) => (
        <>
          <Suspense>
            <SidebarProvider>
              <AppSidebar variant="inset"/>
              <SidebarInset class="h-dvh w-full">
                <AppNav />
                {props.children}
              </SidebarInset>
            </SidebarProvider>
          </Suspense>
        </>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
