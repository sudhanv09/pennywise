import { db } from './app_db';
import {
    transactions,
    categories,
    accounts,
    goals,
    subscriptions,
    loans,
    userSettings
} from '../db/schema';
import { eq, desc, asc, and, gte, lte, sql } from 'drizzle-orm';
import type { TransactionFormData } from '../types/transaction';
import type { CategoryFormData } from '../types/category';
import type { AccountFormData } from '../types/account';
import type { GoalFormData } from '../types/goal';
import type { SubscriptionFormData } from '../types/subscription';
import type { LoanFormData } from '../types/loan';
import type { UserSettingsFormData } from '../types/settings';

// TRANSACTION OPERATIONS

export async function createTransaction(data: TransactionFormData) {
    const [result] = await db.insert(transactions).values(data).returning();
    return result;
}

export async function getAllTransactions() {
    return await db
        .select()
        .from(transactions)
        .leftJoin(categories, eq(transactions.categoryId, categories.id))
        .leftJoin(accounts, eq(transactions.accountId, accounts.id))
        .orderBy(desc(transactions.created));
}

export async function getTransactionById(id: string) {
    const result = await db
        .select()
        .from(transactions)
        .leftJoin(categories, eq(transactions.categoryId, categories.id))
        .leftJoin(accounts, eq(transactions.accountId, accounts.id))
        .where(eq(transactions.id, id))
        .limit(1);

    return result[0] || null;
}

export async function updateTransaction(id: string, data: Partial<TransactionFormData>) {
    const [result] = await db.update(transactions).set(data).where(eq(transactions.id, id)).returning();
    return result;
}

export async function deleteTransaction(id: string): Promise<void> {
    await db.delete(transactions).where(eq(transactions.id, id));
}

export async function getTransactionsByDateRange(startDate: string, endDate: string) {
    return await db
        .select()
        .from(transactions)
        .leftJoin(categories, eq(transactions.categoryId, categories.id))
        .leftJoin(accounts, eq(transactions.accountId, accounts.id))
        .where(and(gte(transactions.created, startDate), lte(transactions.created, endDate)))
        .orderBy(desc(transactions.created));
}

// CATEGORY OPERATIONS

export async function getAllCategories() {
    return await db.select().from(categories).orderBy(asc(categories.name));
}

export async function createCategory(data: CategoryFormData) {
    const [result] = await db.insert(categories).values(data).returning();
    return result;
}

export async function updateCategory(id: number, data: Partial<CategoryFormData>) {
    const [result] = await db.update(categories).set(data).where(eq(categories.id, id)).returning();
    return result;
}

export async function deleteCategory(id: number): Promise<void> {
    await db.delete(categories).where(eq(categories.id, id));
}

export async function getCategoryById(id: number) {
    const result = await db.select().from(categories).where(eq(categories.id, id)).limit(1);
    return result[0] || null;
}

// ACCOUNT OPERATIONS

export async function getAllAccounts() {
    return await db.select().from(accounts).orderBy(asc(accounts.name));
}

export async function createAccount(data: AccountFormData) {
    const [result] = await db.insert(accounts).values(data).returning();
    return result;
}

export async function updateAccount(id: string, data: Partial<AccountFormData>) {
    const [result] = await db.update(accounts).set(data).where(eq(accounts.id, id)).returning();
    return result;
}

export async function deleteAccount(id: string): Promise<void> {
    await db.delete(accounts).where(eq(accounts.id, id));
}

export async function getAccountById(id: string) {
    const result = await db.select().from(accounts).where(eq(accounts.id, id)).limit(1);
    return result[0] || null;
}

// GOAL OPERATIONS

export async function getAllGoals() {
    return await db.select().from(goals).orderBy(desc(goals.createdAt));
}

export async function createGoal(data: GoalFormData) {
    const [result] = await db.insert(goals).values({
        ...data,
        currentAmount: 0,
        isCompleted: false
    }).returning();
    return result;
}

export async function updateGoal(id: string, data: Partial<GoalFormData>) {
    const [result] = await db.update(goals).set(data).where(eq(goals.id, id)).returning();
    return result;
}

export async function deleteGoal(id: string): Promise<void> {
    await db.delete(goals).where(eq(goals.id, id));
}

export async function getGoalById(id: string) {
    const result = await db.select().from(goals).where(eq(goals.id, id)).limit(1);
    return result[0] || null;
}

// SUBSCRIPTION OPERATIONS

export async function getAllSubscriptions() {
    return await db
        .select()
        .from(subscriptions)
        .leftJoin(categories, eq(subscriptions.categoryId, categories.id))
        .leftJoin(accounts, eq(subscriptions.accountId, accounts.id))
        .orderBy(desc(subscriptions.createdAt));
}

export async function createSubscription(data: SubscriptionFormData) {
    const [result] = await db.insert(subscriptions).values({
        ...data,
        isActive: true
    }).returning();
    return result;
}

export async function updateSubscription(id: string, data: Partial<SubscriptionFormData>) {
    const [result] = await db.update(subscriptions).set(data).where(eq(subscriptions.id, id)).returning();
    return result;
}

export async function deleteSubscription(id: string): Promise<void> {
    await db.delete(subscriptions).where(eq(subscriptions.id, id));
}

export async function getSubscriptionById(id: string) {
    const result = await db
        .select()
        .from(subscriptions)
        .leftJoin(categories, eq(subscriptions.categoryId, categories.id))
        .leftJoin(accounts, eq(subscriptions.accountId, accounts.id))
        .where(eq(subscriptions.id, id))
        .limit(1);

    return result[0] || null;
}

// LOAN OPERATIONS

export async function getAllLoans() {
    return await db.select().from(loans).orderBy(desc(loans.createdAt));
}

export async function createLoan(data: LoanFormData) {
    const [result] = await db.insert(loans).values({
        ...data,
        remainingAmount: data.principalAmount,
        isActive: true
    }).returning();
    return result;
}

export async function updateLoan(id: string, data: Partial<LoanFormData>) {
    const [result] = await db.update(loans).set(data).where(eq(loans.id, id)).returning();
    return result;
}

export async function deleteLoan(id: string): Promise<void> {
    await db.delete(loans).where(eq(loans.id, id));
}

export async function getLoanById(id: string) {
    const result = await db.select().from(loans).where(eq(loans.id, id)).limit(1);
    return result[0] || null;
}

// USER SETTINGS OPERATIONS

export async function getUserSettings() {
    const result = await db.select().from(userSettings).limit(1);

    if (result.length === 0) {
        const [newSettings] = await db.insert(userSettings).values({}).returning();
        return newSettings;
    }

    return result[0];
}

export async function updateUserSettings(data: Partial<UserSettingsFormData>) {
    const currentSettings = await getUserSettings();
    const [result] = await db
        .update(userSettings)
        .set({ ...data, updatedAt: sql`(CURRENT_TIMESTAMP)` })
        .where(eq(userSettings.id, currentSettings.id))
        .returning();

    return result;
}