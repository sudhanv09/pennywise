import { createRouter as createTanstackRouter } from "@tanstack/solid-router";
import { routeTree } from "./routeTree.gen";

export function createRouter() {
  const router = createTanstackRouter({
    defaultErrorComponent: err => <div>{err.error.stack}</div>,
    routeTree,
    defaultPreload: "intent",
    defaultStaleTime: 5000,
    scrollRestoration: true
  });
  return router;
}

declare module "@tanstack/solid-router" {
  interface Register {
    router: ReturnType<typeof createRouter>;
  }
}
