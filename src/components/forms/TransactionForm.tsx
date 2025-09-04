import { createForm, valiForm } from "@modular-forms/solid";
import * as v from "valibot";
import { Input } from "../ui/Input";
import { Select } from "../ui/Select";
import { ToggleGroup } from "../ui/ToggleGroup";
import { Button } from "../ui/Button";
import styles from "./TransactionForm.module.css";

const categoryOptions = [
  { value: "shopping", label: "Shopping" },
  { value: "bills", label: "Bills" },
  { value: "groceries", label: "Groceries" },
  { value: "entertainment", label: "Entertainment" },
  { value: "transport", label: "Transport" },
  { value: "other", label: "Other" },
] as const;

const paymentMethodOptions = [
  {
    value: "cash",
    label: "Cash",
    icon: (
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
        <line x1="8" y1="21" x2="16" y2="21" />
        <line x1="12" y1="17" x2="12" y2="21" />
      </svg>
    ),
  },
  {
    value: "card",
    label: "Card",
    icon: (
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <rect x="1" y="4" width="22" height="16" rx="2" ry="2" />
        <line x1="1" y1="10" x2="23" y2="10" />
      </svg>
    ),
  },
] as const;

const typeOptions = [
  { value: "normal", label: "Normal" },
  { value: "loan", label: "Loan" },
  { value: "goal", label: "Goal" },
] as const;

const TransactionSchema = v.object({
  title: v.pipe(v.string(), v.minLength(1, "Title is required")),
  amount: v.number(),
  date: v.pipe(v.date()),
  category: v.picklist(categoryOptions.map((c) => c.value)),
  paymentMethod: v.picklist(paymentMethodOptions.map((p) => p.value)),
  type: v.picklist(typeOptions.map((t) => t.value)),
  description: v.optional(v.string()),
});

type TransactionData = v.InferOutput<typeof TransactionSchema>;

export interface TransactionFormProps {
  onSubmit: (transaction: TransactionData) => void;
  initialData?: Partial<TransactionData>;
  mode?: "create" | "edit";
  class?: string;
}

export function TransactionForm(props: TransactionFormProps) {
  const getCurrentDate = () => {
    const now = new Date();
    return now.toISOString().split("T")[0];
  };

  const [form, { Form, Field }] = createForm<TransactionData>({
    validate: valiForm(TransactionSchema),
  });

  const handleSubmit = (values: TransactionData) => {
    props.onSubmit(values);
  };

  const formClassName = () => {
    const baseClass = styles.form;
    const customClass = props.class || "";
    return [baseClass, customClass].filter(Boolean).join(" ");
  };

  return (
    <Form class={formClassName()} onSubmit={handleSubmit}>
      <div class={styles.fieldGroup}>
        <Field name="title">
          {(field, props) => (
            <Input
              {...props}
              label="Transaction Title"
              placeholder="Enter transaction title"
              required
              value={field.value || ""}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>

        <Field name="amount" type="number">
          {(field, props) => (
            <Input
              {...props}
              label="Amount"
              type="number"
              placeholder="0.00"
              required
              value={field.value?.toString() || ""}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>
      </div>

      <div class={styles.fieldGroup}>
        <Field name="date">
          {(field, props) => (
            <Input
              {...props}
              label="Date"
              type="date"
              required
              value={field.value || getCurrentDate()}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>

        <Field name="category">
          {(field, props) => (
            <Select
              {...props}
              label="Category"
              options={categoryOptions}
              placeholder="Select category"
              required
              value={field.value || ""}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>
      </div>

      <div class={styles.fieldGroup}>
        <Field name="paymentMethod">
          {(field, props) => (
            <ToggleGroup
              {...props}
              label="Payment Method"
              options={paymentMethodOptions}
              required
              value={field.value || ""}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>

        <Field name="type">
          {(field, props) => (
            <ToggleGroup
              {...props}
              label="Transaction Type"
              options={typeOptions}
              required
              value={field.value || ""}
              error={field.error}
              class={styles.field}
            />
          )}
        </Field>
      </div>

      <Field name="description">
        {(field, props) => (
          <Input
            {...props}
            label="Description"
            placeholder="Optional description"
            value={field.value || ""}
            error={field.error}
            class={styles.fullWidth}
          />
        )}
      </Field>

      <div class={styles.actions}>
        <Button
          type="submit"
          variant="primary"
          disabled={form.submitting}
          class={styles.submitButton}
        >
          {form.submitting
            ? "Saving..."
            : props.mode === "edit"
            ? "Update Transaction"
            : "Add Transaction"}
        </Button>
      </div>
    </Form>
  );
}
