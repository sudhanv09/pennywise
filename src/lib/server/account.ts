import { prisma } from "$lib/db";

export interface Account {
  id: number;
  name: string;
  currency: string;
  userId: string;
}

export async function getAccounts(): Promise<Account[]> {
  return await prisma.userAccount.findMany();
}

export async function createAccount(
  entry: Omit<Account, "id">
): Promise<Account> {
  return await prisma.userAccount.create({ data: entry });
}

export async function getAccount(id: number): Promise<Account | null> {
  return await prisma.userAccount.findUnique({ where: { id } });
}

export async function updateAccount(
  id: number,
  data: Partial<Omit<Account, "id">>
): Promise<Account> {
  return await prisma.userAccount.update({ where: { id }, data });
}

export async function deleteAccount(id: number): Promise<Account> {
  return await prisma.userAccount.delete({ where: { id } });
}
