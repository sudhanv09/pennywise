import { createFileRoute } from "@tanstack/solid-router";
import styles from "./auth.module.css";

export const Route = createFileRoute("/auth/sign-up")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class={styles.container}>
      <div class={styles.home_button}>
        <button>
          <svg
            height="16"
            width="16"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1024 1024"
          >
            <path d="M874.690416 495.52477c0 11.2973-9.168824 20.466124-20.466124 20.466124l-604.773963 0 188.083679 188.083679c7.992021 7.992021 7.992021 20.947078 0 28.939099-4.001127 3.990894-9.240455 5.996574-14.46955 5.996574-5.239328 0-10.478655-1.995447-14.479783-5.996574l-223.00912-223.00912c-3.837398-3.837398-5.996574-9.046027-5.996574-14.46955 0-5.433756 2.159176-10.632151 5.996574-14.46955l223.019353-223.029586c7.992021-7.992021 20.957311-7.992021 28.949332 0 7.992021 8.002254 7.992021 20.957311 0 28.949332l-188.073446 188.073446 604.753497 0C865.521592 475.058646 874.690416 484.217237 874.690416 495.52477z"></path>
          </svg>
          <span>Back</span>
        </button>
      </div>
      <div class={styles.card}>
        <div class={styles.card__header}>
          <h1>Register</h1>
          <p>Register a new account</p>
        </div>
        <form action="" class={styles.card__body}>
          <div class={styles.input_group}>
            <input id="name" type="text" required />
            <label for="name">Username</label>
          </div>
          <div class={styles.input_group}>
            <input id="email" type="email" required />
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
