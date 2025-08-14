export interface Account {
  id: string;
  name: string;
  type: "bank" | "cash" | "credit" | "investment";
  balance: number;
  currency: string;
  createdAt: string | null;
}

export interface AccountFormData {
  name: string;
  type: "bank" | "cash" | "credit" | "investment";
  balance: number;
  currency: string;
}