import { createFileRoute, Link } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: RouteComponent
});

function RouteComponent() {
  return (
    <main>
      Goto <Link to="/app">app</Link>
    </main>
  );
}
