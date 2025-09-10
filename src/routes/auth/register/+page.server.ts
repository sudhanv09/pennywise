import { superValidate } from "sveltekit-superforms";
import { registerSchema } from "./schema";
import { valibot } from "sveltekit-superforms/adapters";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(valibot(registerSchema)),
    };
};