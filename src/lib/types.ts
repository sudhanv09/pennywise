export enum LoanType {
  BORROWED = "BORROWED",
  LENT = "LENT",
}

export enum RepaymentType {
  ONE_TIME = "ONE_TIME",
  INSTALLMENTS = "INSTALLMENTS",
}

export enum LoanStatus {
  ACTIVE = "ACTIVE",
  PAID = "PAID",
  DEFAULTED = "DEFAULTED",
}

export enum RenewalCycle {
  DAILY = "DAILY",
  WEEKLY = "WEEKLY",
  MONTHLY = "MONTHLY",
  YEARLY = "YEARLY",
}

export enum TransactionType {
  INCOME = "INCOME",
  EXPENSE = "EXPENSE",
  TRANSFER = "TRANSFER"
}

export interface User {
  id: string;
  name: string;
  email: string;
  preferredCurrency: string;
}

export interface Account {
  id: number;
  name: string;
  currency: string;
  userId: string;
}

export interface Transaction {
  id: number;
  title: string;
  description?: string;
  amount: number;
  createdDate: Date;

  userId: string;
  accountId: number;
  categoryId: number;
}

export interface Category {
  id: number;
  name: string;
  userId: string | null;
}

export interface Goal {
  id: number;
  title: string;
  targetAmount: number;
  currentAmount: number;
  deadline?: Date | null;
  userId: string;
}

export interface Loan {
  id: number;
  title: string;
  amount: number;
  interestRate?: number | null;
  startDate: Date;
  endDate?: Date | null;

  type: LoanType;
  repayment: RepaymentType;
  status: LoanStatus;

  userId: string;
}

export interface Subscription {
  id: number;
  name: string;
  amount: number;
  startDate: Date;
  renewalCycle: RenewalCycle;
  nextPayment: Date;
  userId: string;
}

export interface Budget {
  id: number;
  name: string;
  amount: number;
  startDate: Date;
  endDate: Date;
  userId: string;
}
