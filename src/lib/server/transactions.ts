import { prisma } from "$lib/db";
import { type Transaction, TransactionType } from "@/generated/prisma/client";

export async function getTransactions(): Promise<Transaction[]> {
  return await prisma.transaction.findMany();
}

export async function createTransaction(
  entry: Omit<Transaction, "id">
): Promise<Transaction> {
  return await prisma.transaction.create({
    data: {
      ...entry,
    },
  });
}

export async function getTransaction(id: string): Promise<Transaction | null> {
  return await prisma.transaction.findUnique({
    where: { id },
  });
}

export async function getTransactionsByType(
  type: TransactionType
): Promise<Transaction[]> {
  return await prisma.transaction.findMany({
    where: { type },
  });
}

export async function getTransactionsByMonth(
  year: number,
  month: number
): Promise<Transaction[]> {
  const start = new Date(year, month - 1, 1);
  const end = new Date(year, month, 0, 23, 59, 59, 999); // last day of month

  return await prisma.transaction.findMany({
    where: {
      createdDate: {
        gte: start,
        lte: end,
      },
    },
  });
}

export async function getTransactionsByCategory(
  categoryId: number
): Promise<Transaction[]> {
  return await prisma.transaction.findMany({
    where: { categoryId },
  });
}

export async function updateTransaction(
  id: string,
  data: Partial<Omit<Transaction, "id">>
): Promise<Transaction> {
  return await prisma.transaction.update({
    where: { id },
    data,
  });
}

export async function deleteTransaction(id: string): Promise<Transaction> {
  return await prisma.transaction.delete({
    where: { id },
  });
}
