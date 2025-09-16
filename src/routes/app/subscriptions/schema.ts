import * as v from "valibot";
import { RenewalCycle } from "@/generated/prisma/enums";

export const subscriptionSchema = v.object({
  name: v.string(),
  amount: v.number(),
  startDate: v.string(),
  renewalCycle: v.enum(RenewalCycle),
  nextPayment: v.string(),
});