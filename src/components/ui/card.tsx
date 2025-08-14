import { JSX } from "solid-js";
import styles from "./card.module.css";

export interface CardProps {
  title?: string;
  children: JSX.Element;
  class?: string;
}

export function Card(props: CardProps) {
  const classes = () => [
    styles.card,
    props.class
  ].filter(Boolean).join(" ");

  return (
    <div class={classes()}>
      {props.title && (
        <div class={styles.header}>
          <h3 class={styles.title}>{props.title}</h3>
        </div>
      )}
      <div class={styles.content}>
        {props.children}
      </div>
    </div>
  );
}