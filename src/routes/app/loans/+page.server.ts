import { getLoans, createLoan } from "@/lib/server/loans";
import type { PageServerLoad, Actions } from "./$types";
import { superValidate } from "sveltekit-superforms/server";
import { valibot } from "sveltekit-superforms/adapters";
import { loanSchema } from "./schema";
import { fail } from "@sveltejs/kit";
import { invalidateAll } from "$app/navigation";

export const load = (async ({ locals }) => {
  const today = new Date().toISOString().split('T')[0];
  const form = await superValidate({ startDate: today, endDate: today }, valibot(loanSchema));
  return { loans: await getLoans(locals.user!.id), form };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async ({ request, locals }) => {
    const form = await superValidate(request, valibot(loanSchema));

    if (!form.valid) {
      return fail(400, { form });
    }

    const { title, amount, interestRate, startDate, endDate, type, repayment } =
      form.data;

    try {
      await createLoan({
        title,
        amount,
        interestRate: interestRate || null,
        startDate: new Date(startDate),
        endDate: endDate ? new Date(endDate) : null,
        type,
        repayment,
        status: "ACTIVE",
        userId: locals.user.id,
      });

      await invalidateAll();

      return { form };
    } catch (error) {
      return fail(500, { form, error: "Failed to create loan" });
    }
  },
};
