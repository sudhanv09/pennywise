<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Input } from "$lib/components/ui/input";
  import { TransactionType, type Category } from "$lib/types";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ArrowLeftRight from "@lucide/svelte/icons/arrow-left-right";
  import { Separator } from "$lib/components/ui/separator";
  import * as Form from "$lib/components/ui/form";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";
  import { TransactionSchema } from "./schema";
  import {
    superForm,
    type Infer,
    type SuperValidated,
  } from "sveltekit-superforms";
  import { valibotClient } from "sveltekit-superforms/adapters";

  let {
    data,
  }: {
    data: {
      form: SuperValidated<Infer<typeof TransactionSchema>>;
      category: Category[];
    };
  } = $props();

  const form = superForm(data.form, {
    validators: valibotClient(TransactionSchema),
  });

  const { form: formData, enhance } = form;
</script>

<main class="min-h-screen flex flex-col justify-center items-center gap-16">
  <header>
    <h1 class="text-3xl font-bold">New Transaction</h1>
    <p class="text-neutral-500">Add a new transaction to your records</p>
  </header>

  <form method="post" use:enhance>
    <!-- Select transaction type -->
    <div class="flex justify-center mb-6">
      <Form.Field {form} name="type">
        <Form.Control>
          {#snippet children({ props })}
            <ToggleGroup.Root
              type="single"
              {...props}
              bind:value={$formData.type}
            >
              <ToggleGroup.Item
                value={TransactionType.EXPENSE}
                aria-label="Expense"
              >
                <ChevronDown class="size-6" /> Expense
              </ToggleGroup.Item>
              <Separator orientation="vertical" />
              <ToggleGroup.Item
                value={TransactionType.INCOME}
                aria-label="Income"
              >
                <ChevronUp class="size-6" /> Income
              </ToggleGroup.Item>
              <Separator orientation="vertical" />
              <ToggleGroup.Item
                value={TransactionType.TRANSFER}
                aria-label="Transfer"
              >
                <ArrowLeftRight class="size-6" /> Transfer
              </ToggleGroup.Item>
            </ToggleGroup.Root>
          {/snippet}
        </Form.Control>
      </Form.Field>
    </div>

    <!-- Select category and amount -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <Form.Field {form} name="category">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Category</Form.Label>
            <Select.Root
              type="single"
              bind:value={$formData.category}
              name={$formData.category}
            >
              <Select.Trigger {...props}>
                {$formData.category ? $formData.category : "Select a category"}
              </Select.Trigger>
              <Select.Content>
                {#each data.category as item}
                  <Select.Item value={item.name} label={item.name} />
                {/each}
              </Select.Content>
            </Select.Root>
          {/snippet}
        </Form.Control>
      </Form.Field>
      <Form.Field {form} name="amount">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Amount</Form.Label>
            <Input {...props} bind:value={$formData.amount} type="number" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </div>

    <!-- Select date and time -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <Form.Field {form} name="date">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Date</Form.Label>
            <Input {...props} bind:value={$formData.date} type="date" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field {form} name="time">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Time</Form.Label>
            <Input {...props} bind:value={$formData.time} type="time" />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </div>

    <!-- Meta: Goal, loan, subscription -->
    <div class="grid grid-rows-1 md:grid-rows-3 gap-4 mb-6">
      <Form.Field {form} name="account">
        <Form.Control>
          {#snippet children({ props })}
            <ToggleGroup.Root
              {...props}
              type="single"
              bind:value={$formData.account}
            >
              <ToggleGroup.Item
                value={TransactionType.EXPENSE}
                aria-label="Expense"
              >
                <ChevronDown class="size-6" /> Expense
              </ToggleGroup.Item>
            </ToggleGroup.Root>
          {/snippet}
        </Form.Control>
      </Form.Field>
    </div>

    <!-- Add title and description -->
    <div class="space-y-4">
      <Form.Field {form} name="title">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Title</Form.Label>
            <Input {...props} bind:value={$formData.title} />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Field {form} name="description">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Description</Form.Label>
            <Input {...props} bind:value={$formData.description} />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
    </div>

    <div class="flex justify-center pt-4">
      <Form.Button size="lg" class="w-full max-w-xs"
        >Create Transaction</Form.Button
      >
    </div>
  </form>
</main>
