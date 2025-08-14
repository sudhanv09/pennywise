export interface Loan {
  id: string;
  name: string;
  principalAmount: number;
  remainingAmount: number;
  interestRate: number;
  monthlyPayment: number;
  startDate: string;
  endDate?: string | null;
  lenderName?: string | null;
  isActive: boolean;
  createdAt: string;
}

export interface LoanFormData {
  name: string;
  principalAmount: number;
  interestRate: number;
  monthlyPayment: number;
  startDate: string;
  endDate?: string;
  lenderName?: string;
}