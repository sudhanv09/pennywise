import * as v from "valibot"
import { TransactionType } from "$lib/types";

function getCurrentDate() {
  return new Date().toISOString().split("T")[0]; // YYYY-MM-DD
}

function getCurrentTime() {
  return new Date().toISOString().split("T")[1].slice(0, 8); // HH:mm:ss
}

export const TransactionSchema = v.object({
  title: v.string(),
  description: v.optional(v.string()),
  amount: v.pipe(v.number(), v.minValue(1)),
  type: v.optional(v.enum(TransactionType), TransactionType.EXPENSE),
  category: v.string(),
  account: v.optional(v.string(), "Cash"),
  goal: v.optional(v.string(), "No Goal"),
  loan: v.optional(v.string(), "No Loans"),
  date: v.optional(v.pipe(v.string(), v.isoDate()), getCurrentDate),
  time: v.optional(v.pipe(v.string(), v.isoTime()), getCurrentTime),
});
