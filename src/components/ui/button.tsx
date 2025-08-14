import { JSX, splitProps } from "solid-js";
import styles from "./button.module.css";

export interface ButtonProps {
  variant?: "primary" | "secondary" | "outline" | "ghost";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  onClick?: () => void;
  children: JSX.Element;
  class?: string;
  type?: "button" | "submit" | "reset";
}

export function Button(props: ButtonProps) {
  const [local, others] = splitProps(props, [
    "variant",
    "size", 
    "disabled",
    "onClick",
    "children",
    "class"
  ]);

  const variant = local.variant || "primary";
  const size = local.size || "md";

  const classes = () => [
    styles.button,
    styles[variant],
    styles[size],
    local.disabled && styles.disabled,
    local.class
  ].filter(Boolean).join(" ");

  return (
    <button
      class={classes()}
      disabled={local.disabled}
      onClick={local.onClick}
      {...others}
    >
      {local.children}
    </button>
  );
}