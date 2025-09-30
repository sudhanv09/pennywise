import { getAccounts, createAccount } from '@/lib/server/account';
import type { PageServerLoad, Actions } from './$types';
import { superValidate } from "sveltekit-superforms/server";
import { valibot } from "sveltekit-superforms/adapters";
import { fail } from "@sveltejs/kit";
import { accountSchema } from "./schema";
import { toast } from "svelte-sonner";


export const load = (async ({ locals }) => {
    const form = await superValidate(valibot(accountSchema));
    return { accounts: await getAccounts(locals.user!.id), form };
}) satisfies PageServerLoad;

export const actions: Actions = {
    default: async ({ request, locals }) => {
        const form = await superValidate(request, valibot(accountSchema));

        if (!form.valid) {
            return fail(400, { form });
        }

        const { name, currency } = form.data;

        try {
            await createAccount({
                name,
                currency,
                userId: locals.user!.id,
            });

            toast.success("Account created successfully")

            return { form };
        } catch (error) {
            return fail(500, { form, error: "Failed to create account" });
        }
    },
};