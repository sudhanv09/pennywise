import { getGoals, createGoal } from "@/lib/server/goals";
import type { PageServerLoad, Actions } from "./$types";
import { superValidate } from "sveltekit-superforms/server";
import { valibot } from "sveltekit-superforms/adapters";
import { fail } from "@sveltejs/kit";
import { goalSchema } from "./schema";


export const load = (async () => {
  const form = await superValidate(valibot(goalSchema));
  return { goals: await getGoals(), form };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async ({ request, locals }) => {
    const form = await superValidate(request, valibot(goalSchema));

    if (!form.valid) {
      return fail(400, { form });
    }

    const { title, goalType, targetAmount, currentDate, tillDate } = form.data;

    try {
      await createGoal({
        title,
        GoalType: goalType,
        targetAmount,
        currentAmount: 0,
        currentDate: new Date(currentDate),
        tillDate: new Date(tillDate),
        userId: locals.user.id,
      });

      return { form };
    } catch (error) {
      return fail(500, { form, error: "Failed to create goal" });
    }
  },
};
