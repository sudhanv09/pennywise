import { prisma } from "$lib/db";

export interface User {
  id: string;
  name: string;
  email: string;
  preferredCurrency: string;
}
