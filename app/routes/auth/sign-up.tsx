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
        <form action="" class={styles.card__body}>
          <div class={styles.input_group}>
            <input id="name" type="text" placeholder="johndoe" required />
            <label for="name">Username</label>
          </div>
          <div class={styles.input_group}>
            <input
              id="email"
              type="email"
              placeholder="john.doe@email.com"
              required
            />
            <label for="email">Email</label>
          </div>
          <div class={styles.input_group}>
            <input id="password" type="password" required />
            <label for="password">Password</label>
          </div>
          <div class={styles.input_group}>
            <input id="confirm" type="password" required />
            <label for="confirm">Confirm Password</label>
          </div>
          <button>Submit</button>
        </form>
        <div class={styles.card__footer}>
          <p>
            Already have an account? <a href="/auth/login">Login</a>
          </p>
        </div>
      </div>
    </div>
  );
}
