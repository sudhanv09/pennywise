import { prisma } from "$lib/db";
import type { Subscription } from "@/generated/prisma/client";

export async function getSubscriptions(): Promise<Subscription[]> {
  return await prisma.subscription.findMany();
}

export async function createSubscription(
  entry: Omit<Subscription, "id">
): Promise<Subscription> {
  return await prisma.subscription.create({ data: entry });
}

export async function getSubscription(
  id: number
): Promise<Subscription | null> {
  return await prisma.subscription.findUnique({ where: { id } });
}

export async function updateSubscription(
  id: number,
  data: Partial<Omit<Subscription, "id">>
): Promise<Subscription> {
  return await prisma.subscription.update({ where: { id }, data });
}

export async function deleteSubscription(id: number) {
  return await prisma.subscription.delete({ where: { id } });
}
