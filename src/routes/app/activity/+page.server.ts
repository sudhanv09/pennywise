import { getTransactions } from "$lib/server/transactions";
import type { PageServerLoad } from "./$types";

const DAYS_BACK = 30;

export const load = (async ({ locals }) => {
  const userId = locals.user?.id;

  if (!userId) {
    return {
      transactions: [],
    };
  }

  const transactions = await getTransactions();

  const now = new Date();
  const start = new Date(now);
  start.setHours(0, 0, 0, 0);
  start.setDate(start.getDate() - DAYS_BACK);

  const recentTransactions = transactions
    .filter((transaction) => {
      if (transaction.userId !== userId) {
        return false;
      }

      const created = new Date(transaction.createdDate);

      return created >= start && created <= now;
    })
    .sort(
      (a, b) =>
        new Date(b.createdDate).getTime() -
        new Date(a.createdDate).getTime()
    )
    .map((transaction) => {
      const created = new Date(transaction.createdDate);

      return {
        id: transaction.id,
        title: transaction.title,
        amount: transaction.amount,
        type: transaction.type,
        createdDate: created.toISOString(),
      };
    });

  return {
    transactions: recentTransactions,
  };
}) satisfies PageServerLoad;