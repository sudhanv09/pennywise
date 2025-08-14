import { relations, sql } from "drizzle-orm";
import { sqliteTable, text, real, blob, integer } from "drizzle-orm/sqlite-core";
import { ulid } from "ulid";

const transactionType = text('transaction_type', { enum: ["Income", "Expense", "Transfer"] }).notNull()

const specialTag = text('special_tag', {
    enum: ['default', 'recurring', 'goal', 'subscription', 'loan']
}).notNull().default('default');

export const transactions = sqliteTable('transactions', {
    id: text('id').primaryKey().$default(ulid),
    title: text('title'),
    description: text('description'),
    amount: real('amount').notNull(),
    type: transactionType,
    meta: specialTag,
    picture: blob('attachment'),
    created: text('created').default(sql`(CURRENT_TIMESTAMP)`),

    accountId: text('account_id').references(() => accounts.id),
    categoryId: integer('category_id').references(() => categories.id),
    goalId: text('goal_id').references(() => goals.id),
    subscriptionId: text('subscription_id').references(() => subscriptions.id),
    loanId: text('loan_id').references(() => loans.id),
})

export const categories = sqliteTable('categories', {
    id: integer('id').primaryKey({ autoIncrement: true }),
    name: text('name'),
    icon: text('icon'),
    color: text('color', { length: 7 }),
})


export const accounts = sqliteTable('accounts', {
    id: text('id').primaryKey().$default(ulid),
    name: text('name').notNull(),
    type: text('type', { enum: ['bank', 'cash', 'credit', 'investment'] }).notNull(),
    balance: real('balance').notNull().default(0),
    currency: text('currency', { length: 3 }).notNull().default('USD'),
    createdAt: text('created').default(sql`(CURRENT_TIMESTAMP)`),
});

// Goals table
export const goals = sqliteTable('goals', {
    id: text('id').primaryKey().$default(ulid),
    name: text('name').notNull(),
    targetAmount: real('target_amount').notNull(),
    currentAmount: real('current_amount').notNull().default(0),
    targetDate: text('target_date'),
    description: text('description'),
    createdAt: text('created_at').default(sql`(CURRENT_TIMESTAMP)`),
    isCompleted: integer('is_completed', { mode: 'boolean' }).notNull().default(false),
});

// Subscriptions table
export const subscriptions = sqliteTable('subscriptions', {
    id: text('id').primaryKey().$default(ulid),
    name: text('name').notNull(),
    amount: real('amount').notNull(),
    frequency: text('frequency', { enum: ['weekly', 'monthly', 'quarterly', 'yearly'] }).notNull(),
    nextPaymentDate: text('next_payment_date').notNull(),
    categoryId: integer('category_id').notNull().references(() => categories.id),
    accountId: text('account_id').notNull().references(() => accounts.id),
    isActive: integer('is_active', { mode: 'boolean' }).notNull().default(true),
    createdAt: text('created_at').default(sql`(CURRENT_TIMESTAMP)`),
});

// Loans table
export const loans = sqliteTable('loans', {
    id: text('id').primaryKey().$default(ulid),
    name: text('name').notNull(),
    principalAmount: real('principal_amount').notNull(),
    remainingAmount: real('remaining_amount').notNull(),
    interestRate: real('interest_rate').notNull(),
    monthlyPayment: real('monthly_payment').notNull(),
    startDate: text('start_date').notNull(),
    endDate: text('end_date'),
    lenderName: text('lender_name'),
    isActive: integer('is_active', { mode: 'boolean' }).notNull().default(true),
    createdAt: text('created_at').default(sql`(CURRENT_TIMESTAMP)`),
});

// User settings table
export const userSettings = sqliteTable('user_settings', {
    id: integer('id').primaryKey({ autoIncrement: true }),
    defaultCurrency: text('default_currency', { length: 3 }).notNull().default('USD'),
    dateFormat: text('date_format').notNull().default('MM/DD/YYYY'),
    theme: text('theme', { enum: ['light', 'dark', 'system'] }).notNull().default('system'),
    updatedAt: text('updated_at').default(sql`(CURRENT_TIMESTAMP)`),
});

// Relations
export const transactionRelations = relations(transactions, ({ one }) => ({
    account: one(accounts, {
        fields: [transactions.accountId],
        references: [accounts.id],
    }),
    category: one(categories, {
        fields: [transactions.categoryId],
        references: [categories.id],
    }),
    goal: one(goals, {
        fields: [transactions.goalId],
        references: [goals.id],
    }),
    subscription: one(subscriptions, {
        fields: [transactions.subscriptionId],
        references: [subscriptions.id],
    }),
    loan: one(loans, {
        fields: [transactions.loanId],
        references: [loans.id],
    }),
}));

export const categoryRelations = relations(categories, ({ many }) => ({
    transactions: many(transactions),
    subscriptions: many(subscriptions),
}));

export const goalRelations = relations(goals, ({ many }) => ({
    transactions: many(transactions),
}));

export const subscriptionRelations = relations(subscriptions, ({ one, many }) => ({
    category: one(categories, {
        fields: [subscriptions.categoryId],
        references: [categories.id],
    }),
    account: one(accounts, {
        fields: [subscriptions.accountId],
        references: [accounts.id],
    }),
    transactions: many(transactions),
}));

export const loanRelations = relations(loans, ({ many }) => ({
    transactions: many(transactions),
}));

export const accountRelations = relations(accounts, ({ many }) => ({
    transactions: many(transactions),
    subscriptions: many(subscriptions),
}));