<script lang="ts">
  import {
    Card,
    CardHeader,
    CardTitle,
    CardDescription,
    CardContent,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Form from "$lib/components/ui/form";
  import * as Alert from "$lib/components/ui/alert/index.js";
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";

  import { loginSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { valibotClient } from "sveltekit-superforms/adapters";

  let { data }: { data: { form: SuperValidated<Infer<typeof loginSchema>> } } =
    $props();

  const form = superForm(data.form, {
    validators: valibotClient(loginSchema),
    resetForm: false,
  });

  const { form: formData, enhance, message } = form;
</script>

<div
  class="bg-muted flex min-h-svh flex-col items-center justify-center gap-6 p-6 md:p-10"
>
  <div class="flex w-full max-w-sm flex-col gap-6">
    <a href="/" class="flex items-center gap-2 self-center font-medium">
      Pennywise
    </a>
    {#if $message}
      <Alert.Root variant="destructive">
        <CircleAlertIcon class="size-4" />
        <Alert.Title>Error</Alert.Title>
        <Alert.Description>{$message}</Alert.Description>
      </Alert.Root>
    {/if}
    <div class="flex flex-col gap-6">
      <Card>
        <CardHeader class="text-center">
          <CardTitle class="text-xl">Welcome back</CardTitle>
          <CardDescription>Login with Google account</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid gap-3">
            <div class="flex flex-col gap-4">
              <Button variant="outline" class="w-full">
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
                Or continue with
              </span>
            </div>

            <form method="post" use:enhance class="space-y-3">
              <div class="grid gap-6">
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
                      <div class="flex items-center">
                        <Form.Label>Password</Form.Label>
                        <a
                          href="#"
                          class="ml-auto text-sm underline-offset-4 hover:underline"
                        >
                          Forgot your password?
                        </a>
                      </div>
                      <Input
                        {...props}
                        bind:value={$formData.password}
                        type="password"
                      />
                    {/snippet}
                  </Form.Control>
                  <Form.FieldErrors />
                </Form.Field>
                <Form.Button class="w-full">Login</Form.Button>
              </div>
              <div class="text-center text-sm">
                Don&apos;t have an account?{" "}
                <a href="/auth/register" class="underline underline-offset-4">
                  Sign up
                </a>
              </div>
            </form>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</div>
