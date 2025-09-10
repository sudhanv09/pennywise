import type { PageServerLoad } from './$types';
import { superValidate } from "sveltekit-superforms";
import { valibot } from "sveltekit-superforms/adapters";
import { TransactionSchema } from './schema';
import { getAccounts, getCategories, getGoals, getLoans } from '@/lib/server/user';

export const load = (async () => {
    return {
        form: await superValidate(valibot(TransactionSchema)),
        categories: await getCategories(),
        goals: await getGoals(),
        loans: await getLoans(),
        accounts: await getAccounts(),
    };
}) satisfies PageServerLoad;