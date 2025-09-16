import { getSubscriptions, createSubscription } from "@/lib/server/subscriptions";
import type { PageServerLoad, Actions } from "./$types";
import { superValidate } from "sveltekit-superforms/server";
import { valibot } from "sveltekit-superforms/adapters";
import { subscriptionSchema } from "./schema";
import { fail } from "@sveltejs/kit";
import { invalidateAll } from "$app/navigation";

export const load = (async () => {
  const form = await superValidate(valibot(subscriptionSchema));
  return { subscriptions: await getSubscriptions(), form };
}) satisfies PageServerLoad;

export const actions: Actions = {
  default: async ({ request, locals }) => {
    const form = await superValidate(request, valibot(subscriptionSchema));

    if (!form.valid) {
      return fail(400, { form });
    }

    const { name, amount, startDate, renewalCycle, nextPayment } = form.data;

    try {
      await createSubscription({
        name,
        amount,
        startDate: new Date(startDate),
        renewalCycle,
        nextPayment: new Date(nextPayment),
        userId: locals.user.id,
      });

      await invalidateAll();

      return { form };
    } catch (error) {
      return fail(500, { form, error: 'Failed to create subscription' });
    }
  },
};
