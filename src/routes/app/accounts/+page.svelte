<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";

  import Plus from "@lucide/svelte/icons/plus";

  let { data } = $props();
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
    <Dialog.Root>
      <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus /> Add</Dialog.Trigger
      >
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add a new Account</Dialog.Title>
        </Dialog.Header>
        <div class="grid gap-4 py-4">
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="name" class="text-right">Name</Label>
            <Input id="name" value="Pedro Duarte" class="col-span-3" />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="username" class="text-right">Currency</Label>
            <Input id="username" value="@peduarte" class="col-span-3" />
          </div>
        </div>
        <Dialog.Footer>
          <Button type="submit">Save changes</Button>
        </Dialog.Footer>
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
