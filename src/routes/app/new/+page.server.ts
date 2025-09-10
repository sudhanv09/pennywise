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

export const load = (async () => {
  return {
    form: await superValidate(valibot(TransactionSchema)),
    categories: await getCategories(),
    goals: await getGoals(),
    loans: await getLoans(),
    accounts: await getAccounts(),
  };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, valibot(TransactionSchema));

    if (!form.valid) {
      return fail(400);
    }

    console.log(form.data)
  },
};
