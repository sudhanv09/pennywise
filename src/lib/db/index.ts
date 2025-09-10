import { env } from "$env/dynamic/private"
import { PrismaClient } from "@/generated/prisma/client"
export const prisma = new PrismaClient({
    datasources: {
        db: {
            url: env.DATABASE_URL
        }
    }
})