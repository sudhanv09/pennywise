export interface Transaction {
  id: string;
  title: string | null;
  description: string | null;
  amount: number;
  type: "Income" | "Expense" | "Transfer";
  meta: "default" | "recurring" | "goal" | "subscription" | "loan";
  picture: Uint8Array | null;
  created: string | null;
  accountId: string | null;
  categoryId: number | null;
  goalId?: string | null;
  subscriptionId?: string | null;
  loanId?: string | null;
  category?: Category;
  account?: Account;
}

export interface TransactionFormData {
  title: string;
  description: string;
  amount: number;
  type: "Income" | "Expense";
  categoryId: number;
  accountId: string;
  meta: "default" | "recurring" | "goal" | "subscription" | "loan";
}

export interface GroupedTransaction {
  date: string;
  transactions: Transaction[];
  dayTotal: number;
}

// Import types that Transaction depends on
import type { Category } from "./category";
import type { Account } from "./account";