<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import currencies from "$lib/assets/currency.json";
  import * as Select from "$lib/components/ui/select/index.js";

  import Plus from "@lucide/svelte/icons/plus";

  import * as Form from "$lib/components/ui/form/index.js";
  import { superForm } from "sveltekit-superforms";

  let { data } = $props();
  const currencyList = Object.values(currencies);

  const form = superForm(data.form);
  const { form: formData, enhance } = form;

  let dialogOpen = $state(false);

  const triggerContent = $derived(
    currencyList.find((c) => c.code === $formData.currency)?.code ?? "Select a currency"
  );
</script>

<div class="container mx-auto mt-12">
  <header class="flex items-center justify-between">
    <div class="space-y-1">
      <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0">
        Accounts
      </h2>
      <p class="leading-7 [&:not(:first-child)]:mt-2 text-neutral-600">
        Manage your bank accounts here
      </p>
    </div>
    <Dialog.Root bind:open={dialogOpen}>
      <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus /> Add</Dialog.Trigger
      >
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add a new Account</Dialog.Title>
        </Dialog.Header>
        <form method="POST" use:enhance>
          <div class="grid gap-4 py-4">
            <Form.Field {form} name="name">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label for="name" class="text-right">Name</Form.Label>
                    <Input
                      {...props}
                      id="name"
                      class="col-span-3"
                      bind:value={$formData.name}
                    />
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="currency">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label for="currency" class="text-right">Currency</Form.Label>
                    <Select.Root
                      type="single"
                      name="currency"
                      bind:value={$formData.currency}
                    >
                      <Select.Trigger class="w-[180px] col-span-3">
                        {triggerContent}
                      </Select.Trigger>
                      <Select.Content>
                        <Select.Group>
                          <Select.Label>Currencies</Select.Label>
                          {#each currencyList as currency (currency.code)}
                            <Select.Item value={currency.code} label={currency.name}>
                              {currency.code}
                            </Select.Item>
                          {/each}
                        </Select.Group>
                      </Select.Content>
                    </Select.Root>
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
          </div>
          <Dialog.Footer>
            <Form.Button type="submit">Save changes</Form.Button>
          </Dialog.Footer>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  </header>
  <Separator />

  <ul>
    {#if data.accounts.length === 0}
      <p class="text-neutral-500 text-center mt-24">
        No accounts found. Add a new one
      </p>
    {:else}
      {#each data.accounts as account}
        <li>{account.name}</li>
      {/each}
    {/if}
  </ul>
</div>
