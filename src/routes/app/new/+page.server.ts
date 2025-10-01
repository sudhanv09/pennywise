import type { PageServerLoad, Actions } from "./$types";
import { superValidate } from "sveltekit-superforms";
import { valibot } from "sveltekit-superforms/adapters";
import { TransactionSchema } from "./schema";
import { fail } from "@sveltejs/kit";

import { getCategories } from "@/lib/server/categories";
import { getGoals } from "@/lib/server/goals";
import { getLoans } from "@/lib/server/loans";
import { getAccounts } from "@/lib/server/account";
import { createTransaction } from "@/lib/server/transactions";

export const load = (async ({ locals }) => {
  return {
    form: await superValidate(valibot(TransactionSchema)),
    categories: await getCategories(),
    goals: await getGoals(locals.user.id),
    loans: await getLoans(locals.user.id),
    accounts: await getAccounts(locals.user.id),
  };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, valibot(TransactionSchema));

    if (!form.valid) {
      return fail(400, { form });
    }

    const {
      title,
      amount,
      category,
      date,
      time,
      type,
      account,
      description,
    } = form.data;

    const userId = event.locals.user.id;

    try {
      const createdDate = new Date(`${date}${time}`);

      await createTransaction({
        title,
        description: description ?? null,
        amount: amount,
        createdDate,
        type,
        userId,
        accountId: account || 0, // cash
        categoryId: category,
      });

      return { form };
    } catch (error) {
      console.error("Failed to create transaction", error);
      return fail(500, {
        form,
        error: "Failed to create transaction",
      });
    }
  },
};
