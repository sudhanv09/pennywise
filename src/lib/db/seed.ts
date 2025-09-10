import { prisma } from "@/lib/db"

const DEFAULT_CATEGORIES = [
    "Groceries",
    "Shopping",
    "Transport",
    "Home",
    "Personal",
    "Health",
    "Entertainment",
    "Utilities",
    "Education",
    "Other",
]

export async function seedCategories() {
    for (const name of DEFAULT_CATEGORIES) {
        await prisma.category.upsert({
            where: { name },
            update: {},
            create: { name, userId: null },
        })

    }

    console.log("✅ Default categories seeded")
}

async function main() {
    await seedCategories()
}

main()
    .catch((e) => {
        console.error(e)
        process.exit(1)
    })
    .finally(async () => {
        await prisma.$disconnect()
    })
