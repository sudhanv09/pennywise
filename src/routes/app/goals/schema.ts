import * as v from "valibot";
import { GoalType } from "@/generated/prisma/enums";

export const goalSchema = v.object({
  title: v.string(),
  goalType: v.enum(GoalType),
  targetAmount: v.number(),
  currentDate: v.string(),
  tillDate: v.string(),
});
