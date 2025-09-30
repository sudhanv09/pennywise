import { prisma } from "$lib/db";
import type { UserAccount } from "@/generated/prisma/client";

export async function getAccounts(userId: string): Promise<(UserAccount & { balance: number })[]> {
  const accounts = await prisma.userAccount.findMany({
    where: { userId }
  });

  const accountsWithBalance = await Promise.all(
    accounts.map(async (account) => {
      const income = await prisma.transaction.aggregate({
        where: {
          accountId: account.id,
          type: 'Income'
        },
        _sum: { amount: true }
      });

      const expense = await prisma.transaction.aggregate({
        where: {
          accountId: account.id,
          type: 'Expense'
        },
        _sum: { amount: true }
      });

      const balance = (income._sum.amount || 0) - (expense._sum.amount || 0);

      return { ...account, balance };
    })
  );

  return accountsWithBalance;
}

export async function createAccount(
  entry: Omit<UserAccount, "id">
): Promise<UserAccount> {
  return await prisma.userAccount.create({ data: entry });
}

export async function getAccount(id: number): Promise<UserAccount | null> {
  return await prisma.userAccount.findUnique({ where: { id } });
}

export async function updateAccount(
  id: number,
  data: Partial<Omit<UserAccount, "id">>
): Promise<UserAccount> {
  return await prisma.userAccount.update({ where: { id }, data });
}

export async function deleteAccount(id: number): Promise<UserAccount> {
  return await prisma.userAccount.delete({ where: { id } });
}
