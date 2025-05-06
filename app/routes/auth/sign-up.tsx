import { createFileRoute } from "@tanstack/solid-router";
import styles from "./auth.module.css";

export const Route = createFileRoute("/auth/sign-up")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class={styles.container}>
      <div class={styles.card}>
        <div class={styles.card__header}>
          <h1>Register</h1>
          <p>Register a new account</p>
        </div>
        <div class={styles.card__body}>
          <form action="">
            <div class={styles.card__field}>
              <label for="name">Username</label>
              <input id="name" type="text" placeholder="johndoe" />
            </div>
            <div class={styles.card__field}>
              <label for="email">Email</label>
              <input id="email" type="email" placeholder="john.doe@email.com" />
            </div>
            <div class={styles.card__field}>
              <label for="password">Password</label>
              <input id="password" type="password" />
            </div>
            <div class={styles.card__field}>
              <label for="confirm">Confirm Password</label>
              <input id="confirm" type="password" />
            </div>
            <button>Submit</button>
          </form>
        </div>
        <div class={styles.card__footer}>
          <p>
            Already have an account? <a href="/auth/login">Login</a>
          </p>
        </div>
      </div>
    </div>
  );
}
