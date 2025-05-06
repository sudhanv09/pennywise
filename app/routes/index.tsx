import { createFileRoute } from "@tanstack/solid-router";

export const Route = createFileRoute("/")({
  component: RouteComponent
});

function RouteComponent() {
  return (
    <main>
      <h1>Hello world!</h1>
    </main>
  );
}
