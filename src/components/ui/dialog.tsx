import { JSX } from "solid-js";
import { Dialog as KobalteDialog } from "@kobalte/core/dialog";
import styles from "./dialog.module.css";

export interface DialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  children: JSX.Element;
}

export function Dialog(props: DialogProps) {
  return (
    <KobalteDialog open={props.open} onOpenChange={props.onOpenChange}>
      <KobalteDialog.Portal>
        <KobalteDialog.Overlay class={styles.overlay} />
        <div class={styles.positioner}>
          <KobalteDialog.Content class={styles.content}>
            <div class={styles.header}>
              <KobalteDialog.Title class={styles.title}>
                {props.title}
              </KobalteDialog.Title>
              <KobalteDialog.CloseButton class={styles.closeButton}>
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path d="M18 6L6 18M6 6l12 12" />
                </svg>
              </KobalteDialog.CloseButton>
            </div>
            <div class={styles.body}>
              {props.children}
            </div>
          </KobalteDialog.Content>
        </div>
      </KobalteDialog.Portal>
    </KobalteDialog>
  );
}