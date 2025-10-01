import * as v from "valibot";
import { TransactionType } from "@/generated/prisma/enums";

function getCurrentDate() {
  return new Date().toISOString().split("T")[0]; // YYYY-MM-DD
}

function getCurrentTime() {
  return new Date().toLocaleTimeString()
}

export const TransactionSchema = v.object({
  title: v.string(),
  description: v.optional(v.string()),
  amount: v.pipe(v.number(), v.minValue(1)),
  type: v.optional(v.enum(TransactionType), TransactionType.Expense),
  category: v.pipe(v.string(), v.transform(Number), v.number()),
  account: v.optional(v.pipe(v.string(), v.transform(Number), v.number())),
  goal: v.optional(v.string(), "No Goal"),
  loan: v.optional(v.string(), "No Loans"),
  date: v.optional(v.pipe(v.string(), v.isoDate()), getCurrentDate),
  time: v.optional(v.string(), getCurrentTime),
});
