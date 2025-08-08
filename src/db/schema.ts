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

export const accountRelations = relations(accounts, ({ many }) => ({
    transactions: many(transactions),
}))