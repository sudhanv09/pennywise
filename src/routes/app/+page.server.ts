import { getAccounts } from '@/lib/server/account';
import { prisma } from '$lib/db';
import type { PageServerLoad } from './$types';
import { TransactionType } from '@/generated/prisma/enums';

interface DashboardData {
  totalBalance: number;
  totalIncome: number;
  totalExpense: number;
  preferredCurrency: string;
  accounts: Array<{ id: number; name: string; currency: string; balance: number }>;
  categoryBreakdown: Array<{ category: string; amount: number; percentage: number }>;
  transactionsByDate: Array<{
    date: string;
    transactions: Array<{
      id: string;
      title: string;
      amount: number;
      type: TransactionType;
      category: string;
    }>;
  }>;
}

export const load = (async ({ locals }) => {
  const userId = locals.user!.id;
  
  // Get user's preferred currency
  const user = await prisma.user.findUnique({
    where: { id: userId },
    select: { preferredCurrency: true },
  });
  
  // Get current month's date range
  const now = new Date();
  const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
  const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59, 999);

  // Fetch accounts with balances
  const accounts = await getAccounts(userId);
  const totalBalance = accounts.reduce((sum, acc) => sum + acc.balance, 0);

  // Fetch current month's transactions with category details
  const transactions = await prisma.transaction.findMany({
    where: {
      userId,
      createdDate: {
        gte: startOfMonth,
        lte: endOfMonth,
      },
    },
    include: {
      category: true,
    },
    orderBy: {
      createdDate: 'desc',
    },
  });

  // Calculate totals
  const totalIncome = transactions
    .filter((t) => t.type === 'Income')
    .reduce((sum, t) => sum + t.amount, 0);

  const totalExpense = transactions
    .filter((t) => t.type === 'Expense')
    .reduce((sum, t) => sum + t.amount, 0);

  // Calculate category-wise breakdown (expenses only)
  const categoryMap = new Map<string, number>();
  transactions
    .filter((t) => t.type === 'Expense')
    .forEach((t) => {
      const current = categoryMap.get(t.category.name) || 0;
      categoryMap.set(t.category.name, current + t.amount);
    });

  const categoryBreakdown = Array.from(categoryMap.entries())
    .map(([category, amount]) => ({
      category,
      amount,
      percentage: totalExpense > 0 ? (amount / totalExpense) * 100 : 0,
    }))
    .sort((a, b) => b.amount - a.amount);

  // Group transactions by date
  const transactionsByDateMap = new Map<string, typeof transactions>();
  transactions.forEach((t) => {
    const dateKey = t.createdDate.toISOString().split('T')[0]; // YYYY-MM-DD
    if (!transactionsByDateMap.has(dateKey)) {
      transactionsByDateMap.set(dateKey, []);
    }
    transactionsByDateMap.get(dateKey)!.push(t);
  });

  const transactionsByDate = Array.from(transactionsByDateMap.entries()).map(
    ([date, txns]) => ({
      date,
      transactions: txns.map((t) => ({
        id: t.id,
        title: t.title,
        amount: t.amount,
        type: t.type,
        category: t.category.name,
      })),
    })
  );

  const dashboardData: DashboardData = {
    totalBalance,
    totalIncome,
    totalExpense,
    preferredCurrency: user?.preferredCurrency || 'USD',
    accounts,
    categoryBreakdown,
    transactionsByDate,
  };

  return dashboardData;
}) satisfies PageServerLoad;