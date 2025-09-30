import { prisma } from "$lib/db";
import { type Loan } from "@/generated/prisma/client";

export async function getLoans(userId: string): Promise<Loan[]> {
  return await prisma.loan.findMany({
    where: { userId }
  });
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
