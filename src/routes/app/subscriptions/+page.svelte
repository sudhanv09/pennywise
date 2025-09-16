<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Form from "$lib/components/ui/form/index.js";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";

  import Plus from "@lucide/svelte/icons/plus";
  import { superForm } from "sveltekit-superforms";
  import Calendar from "@lucide/svelte/icons/calendar";
  import Clock from "@lucide/svelte/icons/clock";
  import RotateCcw from "@lucide/svelte/icons/rotate-ccw";
  import TrendingUp from "@lucide/svelte/icons/trending-up";

  let { data } = $props();

  const superFormObj = superForm(data.form);
  const { form, enhance } = superFormObj;

  let dialogOpen = $state(false);
</script>

<div class="container mx-auto mt-12">
  <header class="flex items-center justify-between">
    <div class="space-y-1">
      <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0">
        Subscriptions
      </h2>
      <p class="leading-7 [&:not(:first-child)]:mt-2 text-neutral-600">
        Manage your subscriptions here
      </p>
    </div>
    <Dialog.Root bind:open={dialogOpen}>
      <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus /> Add</Dialog.Trigger
      >
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add a new Subscription</Dialog.Title>
        </Dialog.Header>
        <form method="POST" use:enhance>
          <div class="grid gap-4 py-4">
            <Form.Field form={superFormObj} name="name">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Name</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        bind:value={$form.name}
                      />
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="amount">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Amount</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        type="number"
                        bind:value={$form.amount}
                      />
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="renewalCycle">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Cycle</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <ToggleGroup.Root
                        type="single"
                        {...props}
                        class="col-span-3"
                        bind:value={$form.renewalCycle}
                      >
                        <ToggleGroup.Item value="DAILY">
                          <Clock />
                        </ToggleGroup.Item>
                        <ToggleGroup.Item value="WEEKLY">
                          <Calendar />
                        </ToggleGroup.Item>
                        <ToggleGroup.Item value="MONTHLY">
                          <RotateCcw />
                        </ToggleGroup.Item>
                        <ToggleGroup.Item value="YEARLY">
                          <TrendingUp />
                        </ToggleGroup.Item>
                      </ToggleGroup.Root>
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="startDate">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Start Date</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        type="date"
                        bind:value={$form.startDate}
                      />
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="nextPayment">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Next Payment</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        type="date"
                        bind:value={$form.nextPayment}
                      />
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
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
    {#if data.subscriptions.length === 0}
      <p class="text-neutral-500 text-center mt-24">
        No subscriptions found. Add a new one
      </p>
    {:else}
      {#each data.subscriptions as subscription}
        <li>{subscription.name}</li>
      {/each}
    {/if}
  </ul>
</div>
