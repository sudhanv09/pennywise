import * as v from "valibot"
import { TransactionType } from "$lib/types";

export const TransactionSchema = v.object({
  title: v.string(),
  description: v.optional(v.string()),
  amount: v.number(),
  type: v.enum(TransactionType),
  category: v.string(),
  account: v.string(),
  goal: v.optional(v.string()),
  loan: v.optional(v.string()),
  date: v.pipe(v.string(), v.isoDate()),
  time: v.pipe(v.string(), v.isoTime()),
});
