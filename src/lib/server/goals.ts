import { prisma } from "$lib/db";

export interface Goal {
  id: number;
  title: string;
  targetAmount: number;
  currentAmount: number;
  deadline?: Date | null;
  userId: string;
}

export async function getGoals(): Promise<Goal[]> {
  return await prisma.goal.findMany();
}

export async function createGoal(entry: Omit<Goal, "id">): Promise<Goal> {
  return await prisma.goal.create({ data: entry });
}

export async function getGoal(id: number): Promise<Goal | null> {
  return await prisma.goal.findUnique({ where: { id } });
}

export async function updateGoal(
  id: number,
  data: Partial<Omit<Goal, "id">>
): Promise<Goal> {
  return await prisma.goal.update({ where: { id }, data });
}

export async function deleteGoal(id: number): Promise<Goal> {
  return await prisma.goal.delete({ where: { id } });
}
