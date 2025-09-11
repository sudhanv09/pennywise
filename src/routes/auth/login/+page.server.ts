import { superValidate, message } from "sveltekit-superforms";
import { loginSchema } from "./schema";
import { valibot } from "sveltekit-superforms/adapters";
import type { PageServerLoad, Actions } from "./$types";
import { fail, redirect, error } from "@sveltejs/kit";
import { auth } from "@/lib/auth";
import { APIError } from "better-auth";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(valibot(loginSchema)),
  };
};

export const actions: Actions = {
  default: async ({ request }) => {
    const form = await superValidate(request, valibot(loginSchema));

    if (!form.valid) {
      return fail(400);
    }

    try {
      await auth.api.signInEmail({
        body: {
          email: form.data.email,
          password: form.data.password,
        },
        headers: request.headers
      });

    } catch (err) {
      if (err instanceof APIError) {
        if (err.status === "UNAUTHORIZED") {
          return message(form, "Invalid email or password.", { status: 401 });
        }
        if (err.status === "BAD_REQUEST") {
          return message(form, "Invalid request. Please check your input.", { status: 400 });
        }
      }

      return error(500, "An error occured during registration")
    }

    redirect(303, "/app")

  }
};
