import { superValidate, setError } from "sveltekit-superforms";
import { registerSchema } from "./schema";
import { valibot } from "sveltekit-superforms/adapters";
import type { PageServerLoad, Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { auth } from "@/lib/auth";
import { APIError } from "better-auth";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(valibot(registerSchema)),
  };
};

export const actions: Actions = {
  default: async (event) => {
    console.log(event);
    const form = await superValidate(event, valibot(registerSchema));

    if (!form.valid) {
      return fail(400);
    }

    try {
      await auth.api.signUpEmail({
        body: {
          name: form.data.name,
          email: form.data.email,
          password: form.data.password,
        },
      });
      throw redirect(303, "/app");
    } catch (error) {
      if (error instanceof APIError) {
        return setError(form, "email", "Email already exists");
      }
    }
  },
};
