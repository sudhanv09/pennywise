import { prisma } from "$lib/db";
import type { Budget } from "@/generated/prisma/client";

export async function getBudgets(): Promise<Budget[]> {
  return await prisma.budget.findMany();
}

export async function createBudget(entry: Omit<Budget, "id">): Promise<Budget> {
  return await prisma.budget.create({ data: entry });
}

export async function getBudget(id: number): Promise<Budget | null> {
  return await prisma.budget.findUnique({ where: { id } });
}

export async function updateBudget(
  id: number,
  data: Partial<Omit<Budget, "id">>
): Promise<Budget> {
  return await prisma.budget.update({ where: { id }, data });
}

export async function deleteBudget(id: number): Promise<Budget> {
  return await prisma.budget.delete({ where: { id } });
}
