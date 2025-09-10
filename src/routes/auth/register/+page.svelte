<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";

  import { registerSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { valibotClient } from "sveltekit-superforms/adapters";

  let {
    data,
  }: { data: { form: SuperValidated<Infer<typeof registerSchema>> } } =
    $props();

  const form = superForm(data.form, {
    validators: valibotClient(registerSchema),
  });

  const { form: formData, enhance } = form;
</script>

<div
  class="flex min-h-svh flex-col items-center justify-center gap-6 p-6 md:p-10"
>
  <div class="flex w-full max-w-sm flex-col gap-6">
    <div class="flex flex-col gap-6">
      <div class="text-center">
        <h1 class="text-xl">Create your account</h1>
      </div>
      <div class="grid gap-6">
        <div class="flex flex-col gap-4">
          <Button variant="outline" class="w-full rounded-full">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <path
                d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"
                fill="currentColor"
              />
            </svg>
            Login with Google
          </Button>
        </div>
        <div
          class="after:border-border relative text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t"
        >
          <span class="bg-card text-muted-foreground relative z-10 px-2">
            Or
          </span>
        </div>
        <form method="post" use:enhance>
          <div class="grid gap-3">
            <Form.Field {form} name="name">
              <Form.Control>
                {#snippet children({ props })}
                  <Form.Label>Name</Form.Label>
                  <Input {...props} bind:value={$formData.name} />
                {/snippet}
              </Form.Control>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field {form} name="email">
              <Form.Control>
                {#snippet children({ props })}
                  <Form.Label>Email</Form.Label>
                  <Input {...props} bind:value={$formData.email} />
                {/snippet}
              </Form.Control>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field {form} name="password">
              <Form.Control>
                {#snippet children({ props })}
                  <Form.Label>Password</Form.Label>
                  <Input {...props} bind:value={$formData.password} />
                {/snippet}
              </Form.Control>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field {form} name="confirmPassword">
              <Form.Control>
                {#snippet children({ props })}
                  <Form.Label>Confirm Password</Form.Label>
                  <Input {...props} bind:value={$formData.confirmPassword} />
                {/snippet}
              </Form.Control>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Button class="w-full">Register</Form.Button>
          </div>
        </form>
        <div class="text-center text-sm">
          Already a user?{" "}
          <a href="/auth/login" class="underline underline-offset-4">
            Log in
          </a>
        </div>
      </div>
    </div>
  </div>
</div>
