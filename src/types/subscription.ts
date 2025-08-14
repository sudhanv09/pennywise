export interface Subscription {
  id: string;
  name: string;
  amount: number;
  frequency: "weekly" | "monthly" | "quarterly" | "yearly";
  nextPaymentDate: string;
  categoryId: number;
  accountId: string;
  isActive: boolean;
  createdAt: string;
  category?: Category;
  account?: Account;
}

export interface SubscriptionFormData {
  name: string;
  amount: number;
  frequency: "weekly" | "monthly" | "quarterly" | "yearly";
  nextPaymentDate: string;
  categoryId: number;
  accountId: string;
}

// Import types that Subscription depends on
import type { Category } from "./category";
import type { Account } from "./account";