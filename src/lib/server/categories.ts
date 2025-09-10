import { prisma } from "$lib/db";
import type { Category } from "@/generated/prisma/client";

export async function createCategory(
  entry: Omit<Category, "id">
): Promise<Category> {
  return await prisma.category.create({ data: entry });
}

export async function getCategory(id: number): Promise<Category | null> {
  return await prisma.category.findUnique({ where: { id } });
}

export async function updateCategory(
  id: number,
  data: Partial<Omit<Category, "id">>
): Promise<Category> {
  return await prisma.category.update({ where: { id }, data });
}

export async function deleteCategory(id: number): Promise<Category> {
  return await prisma.category.delete({ where: { id } });
}

export async function getCategories(): Promise<Category[]> {
  return await prisma.category.findMany();
}
