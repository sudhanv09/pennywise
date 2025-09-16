import * as v from "valibot";
import { LoanType, RepaymentType } from "@/generated/prisma/enums";

export const loanSchema = v.object({
  title: v.string(),
  amount: v.number(),
  interestRate: v.optional(v.number()),
  startDate: v.string(),
  endDate: v.optional(v.string()),
  type: v.enum(LoanType),
  repayment: v.enum(RepaymentType),
});