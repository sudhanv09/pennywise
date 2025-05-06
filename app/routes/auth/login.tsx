import { createFileRoute } from "@tanstack/solid-router";
import styles from "./auth.module.css";

export const Route = createFileRoute("/auth/login")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class={styles.container}>
      <div class={styles.card}>
        <div class={styles.card__header}>
          <h1>Login</h1>
          <p>Enter your email to login to your account</p>
        </div>
        <div class={styles.card__body}>
          <form action="">
            <div class={styles.card__field}>
              <label for="email">Email</label>
              <input id="email" type="email" placeholder="john.doe@email.com" />
            </div>
            <div class={styles.card__field}>
              <div>
                <label for="password">Password</label>
                <a href="#">Forgot Password?</a>
              </div>
              <input id="password" type="password" />
            </div>
            <button>Submit</button>
          </form>
        </div>
        <div class={styles.card__footer}>
          <p>
            Don't have an account? <a href="/auth/sign-up">Sign up</a>
          </p>
        </div>
      </div>
    </div>
  );
}
