<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";

  import Plus from "@lucide/svelte/icons/plus";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";

  import * as Form from "$lib/components/ui/form/index.js";
  import { superForm } from "sveltekit-superforms";

  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
  } from "@/lib/components/ui/card/index.js";
  import { Progress } from "$lib/components/ui/progress/index.js";

  let { data } = $props();

  const form = superForm(data.form);
  const { form: formData, enhance } = form;

  let dialogOpen = $state(false);
</script>

<div class="container mx-auto mt-12">
  <header class="flex items-center justify-between">
    <div class="space-y-1">
      <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0">
        Goals
      </h2>
      <p class="leading-7 [&:not(:first-child)]:mt-2 text-neutral-600">
        Manage your goals here
      </p>
    </div>
    <Dialog.Root bind:open={dialogOpen}>
      <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus /> Add</Dialog.Trigger
      >
      <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
          <Dialog.Title>Add a new Goal</Dialog.Title>
        </Dialog.Header>
        <form method="POST" use:enhance>
          <div class="grid gap-4 py-4">
            <Form.Field {form} name="goalType">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">Type</Form.Label>
                    <ToggleGroup.Root
                      type="single"
                      {...props}
                      class="col-span-3"
                      bind:value={$formData.goalType}
                    >
                      <ToggleGroup.Item value="Savings">
                        <ChevronUp class="text-green-600"/> Savings
                      </ToggleGroup.Item>
                      <ToggleGroup.Item value="Expense">
                        <ChevronDown class="text-red-600"/> Expense
                      </ToggleGroup.Item>
                    </ToggleGroup.Root>
                    <input type="hidden" name="goalType" value={$formData.goalType}>
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="title">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">Title</Form.Label>
                    <Input
                      {...props}
                      class="col-span-3"
                      bind:value={$formData.title}
                    />
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="targetAmount">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label>Goal Amount</Form.Label>
                    <Input
                      {...props}
                      class="col-span-3"
                      type="number"
                      bind:value={$formData.targetAmount}
                    />
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="currentDate">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">From</Form.Label>
                    <Input
                      {...props}
                      class="col-span-3"
                      type="date"
                      bind:value={$formData.currentDate}
                    />
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>
            <Form.Field {form} name="tillDate">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">Till</Form.Label>
                    <Input
                      {...props}
                      class="col-span-3"
                      type="date"
                      bind:value={$formData.tillDate}
                    />
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

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-8">
    {#if data.goals.length === 0}
      <div class="col-span-full text-center mt-24">
        <p class="text-neutral-500">
          No goals found. Add a new one
        </p>
      </div>
    {:else}
      {#each data.goals as goal}
        <Card>
          <CardHeader>
            <CardTitle>{goal.title}</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-1">
              <p class="text-sm text-neutral-600">
                Target: {goal.targetAmount.toLocaleString()} by {goal.tillDate.toLocaleDateString()}
              </p>
            </div>
            <Progress value={goal.percentage} class="w-full" />
            <p class="text-xs text-neutral-500">Add {goal.dailyAdd.toFixed(2)} per day</p>
          </CardContent>
        </Card>
      {/each}
    {/if}
  </div>
</div>
