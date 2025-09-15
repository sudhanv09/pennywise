import { getAccounts } from '@/lib/server/account';
import type { PageServerLoad } from './$types';

export const load = (async () => {
    return { accounts: await getAccounts() };
}) satisfies PageServerLoad;