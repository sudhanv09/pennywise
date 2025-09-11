import { superValidate, message } from "sveltekit-superforms";
import { registerSchema } from "./schema";
import { valibot } from "sveltekit-superforms/adapters";
import type { PageServerLoad, Actions } from "./$types";
import { error, fail, redirect } from "@sveltejs/kit";
import { auth } from "@/lib/auth";
import { APIError } from "better-auth";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(valibot(registerSchema)),
  };
};

export const actions: Actions = {
  default: async (event) => {
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
    } catch (err) {
      if (err instanceof APIError) {
        if (err.status === "UNPROCESSABLE_ENTITY") {
          return message(form, err.body?.message, { status: 422 });
        }
        if (err.status === "BAD_REQUEST") {
          return message(form, "Invalid request. Please check your input.", { status: 400 });
        }
      }

      return error(500, "An error occured during registration")
    }

    redirect(303, "/app");
  },
};
