import { createFileRoute } from "@tanstack/solid-router";
import styles from "./index.module.css";

export const Route = createFileRoute("/app/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class={styles.dashboard}>
      <nav>Nav bar add button settings</nav>
      <section class={styles.dashboard__info}>Cards</section>
      <section class={styles.dashboard__table}>Table</section>
    </div>
  );
}
