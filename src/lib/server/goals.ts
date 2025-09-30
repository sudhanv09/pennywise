import { prisma } from "$lib/db";
import type { Goal } from "@/generated/prisma/client";

export async function getGoals(userId: string): Promise<(Goal & { percentage: number; dailyAdd: number })[]> {
  const goals = await prisma.goal.findMany({
    where: { userId }
  });

  const now = new Date();
  const goalsWithProgress = goals.map(goal => {
    const daysLeft = Math.max(1, Math.ceil((goal.tillDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24)));
    const remaining = goal.targetAmount - goal.currentAmount;
    const dailyAdd = remaining / daysLeft;
    const percentage = Math.min(100, Math.max(0, (goal.currentAmount / goal.targetAmount) * 100));

    return { ...goal, percentage, dailyAdd };
  });

  return goalsWithProgress;
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
