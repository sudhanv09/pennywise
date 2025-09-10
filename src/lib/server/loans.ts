import { prisma } from "$lib/db";

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

export async function getLoans(): Promise<Loan[]> {
  return await prisma.loan.findMany();
}

export async function createLoan(entry: Omit<Loan, "id">): Promise<Loan> {
  return await prisma.loan.create({ data: entry });
}

export async function getLoan(id: number): Promise<Loan | null> {
  return await prisma.loan.findUnique({ where: { id } });
}

export async function updateLoan(
  id: number,
  data: Partial<Omit<Loan, "id">>
): Promise<Loan> {
  return await prisma.loan.update({ where: { id }, data });
}

export async function deleteLoan(id: number): Promise<Loan> {
  return await prisma.loan.delete({ where: { id } });
}
