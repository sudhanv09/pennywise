<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Form from "$lib/components/ui/form/index.js";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";

  import Plus from "@lucide/svelte/icons/plus";
  import ArrowLeftRight from "@lucide/svelte/icons/arrow-left-right";
  import Repeat from "@lucide/svelte/icons/repeat";
  import DollarSign from "@lucide/svelte/icons/dollar-sign";
  import Calendar from "@lucide/svelte/icons/calendar";
  import { superForm } from "sveltekit-superforms";

  let { data } = $props();

  const superFormObj = superForm(data.form);
  const { form, enhance } = superFormObj;

  let dialogOpen = $state(false);
</script>

<div class="container mx-auto mt-12">
  <header class="flex items-center justify-between">
    <div class="space-y-1">
      <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0">
        Loans
      </h2>
      <p class="leading-7 [&:not(:first-child)]:mt-2 text-neutral-600">
        Manage your loans here. You can select a 1 time loan or an installment
        based loan.
      </p>
    </div>
    <Dialog.Root bind:open={dialogOpen}>
      <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus /> Add</Dialog.Trigger
      >
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add a new Loan</Dialog.Title>
        </Dialog.Header>
        <form method="POST" use:enhance>
          <div class="grid gap-4 py-4">
            <Form.Field form={superFormObj} name="type">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Type</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <ToggleGroup.Root
                        type="single"
                        {...props}
                        class="col-span-3"
                        bind:value={$form.type}
                      >
                        <ToggleGroup.Item value="BORROWED">
                          <ArrowLeftRight />
                        </ToggleGroup.Item>
                        <ToggleGroup.Item value="LENT">
                          <Repeat />
                        </ToggleGroup.Item>
                      </ToggleGroup.Root>
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="title">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Title</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        bind:value={$form.title}
                      />
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              {/snippet}
            </Form.Field>

            <Form.Field form={superFormObj} name="repayment">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Repayment</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <ToggleGroup.Root
                        type="single"
                        {...props}
                        class="col-span-3"
                        bind:value={$form.repayment}
                      >
                        <ToggleGroup.Item value="ONE_TIME">
                          <DollarSign />
                        </ToggleGroup.Item>
                        <ToggleGroup.Item value="INSTALLMENTS">
                          <Calendar />
                        </ToggleGroup.Item>
                      </ToggleGroup.Root>
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

            <Form.Field form={superFormObj} name="interestRate">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">Interest Rate (%)</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        type="number"
                        step="0.01"
                        bind:value={$form.interestRate}
                      />
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

            <Form.Field form={superFormObj} name="endDate">
              {#snippet children({ errors })}
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Label class="text-right">End Date</Form.Label>
                  <Form.Control>
                    {#snippet children({ props })}
                      <Input
                        {...props}
                        class="col-span-3"
                        type="date"
                        bind:value={$form.endDate}
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
    {#if data.loans.length === 0}
      <p class="text-neutral-500 text-center mt-24">
        No loans found. Add a new one
      </p>
    {:else}
      {#each data.loans as loan}
        <li>{loan.title}</li>
      {/each}
    {/if}
  </ul>
</div>
