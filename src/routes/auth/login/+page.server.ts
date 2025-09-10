import { superValidate } from "sveltekit-superforms";
import { loginSchema } from "./schema";
import { valibot } from "sveltekit-superforms/adapters";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(valibot(loginSchema)),
    };
};