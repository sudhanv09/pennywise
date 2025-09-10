import { prisma } from "$lib/db";
import type { Account, Category, Goal, Loan, LoanStatus, LoanType, RepaymentType } from "$lib/types";


export async function getGoals(): Promise<Goal[]> {
    return await prisma.goal.findMany();
}

export async function getLoans(): Promise<Loan[]> {
    const loans = await prisma.loan.findMany();

    return loans.map(l => ({
        id: l.id,
        title: l.title,
        userId: l.userId,
        amount: l.amount,
        interestRate: l.interestRate,
        startDate: new Date(l.startDate),
        endDate: l.endDate ? new Date(l.endDate) : null,
        type: l.type as LoanType,
        repayment: l.repayment as RepaymentType,
        status: l.status as LoanStatus,
    }));
}


export async function getSubscriptions() {
    return await prisma.subscription.findMany();
}

export async function getCategories(): Promise<Category[]> {
    return await prisma.category.findMany()
}

export async function getAccounts(): Promise<Account[]> {
    return await prisma.userAccount.findMany();
}
