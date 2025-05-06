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
        <form action="" class={styles.card__body}>
          <div class={styles.input_group}>
            <input id="email" type="email" placeholder="john.doe@email.com" required/>
            <label for="email">Email</label>
          </div>
          <div class={styles.input_group}>
            <input id="password" type="password" required/>
            <label for="password">Password</label>
          </div>
          <a href="#">Forgot Password?</a>
          <button>Submit</button>
        </form>
        <div class={styles.card__footer}>
          <p>
            Don't have an account? <a href="/auth/sign-up">Sign up</a>
          </p>
        </div>
      </div>
    </div>
  );
}
