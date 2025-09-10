import { prisma } from "$lib/db";
import type { UserAccount } from "@/generated/prisma/client";

export async function getAccounts(): Promise<UserAccount[]> {
  return await prisma.userAccount.findMany();
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
