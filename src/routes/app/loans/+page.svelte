<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Form from "$lib/components/ui/form/index.js";
  import * as ToggleGroup from "$lib/components/ui/toggle-group/index.js";

  import {
    DateFormatter,
    getLocalTimeZone,
    parseDate,
    toCalendarDate
  } from "@internationalized/date";
  import { cn } from "$lib/utils.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";

  import Plus from "@lucide/svelte/icons/plus";
  import ArrowLeftRight from "@lucide/svelte/icons/arrow-left-right";
  import Repeat from "@lucide/svelte/icons/repeat";
  import DollarSign from "@lucide/svelte/icons/dollar-sign";
  import CalendarIcon from "@lucide/svelte/icons/calendar";
  import { superForm } from "sveltekit-superforms";

  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent,
  } from "@/lib/components/ui/card/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";

  let { data } = $props();

  const superFormObj = superForm(data.form);
  const { form, enhance } = superFormObj;

  let dialogOpen = $state(false);

  let startDateValue = $state(
    $form.startDate ? parseDate($form.startDate) : undefined
  );
  let endDateValue = $state(
    $form.endDate ? parseDate($form.endDate).add({ months: 1 }) : undefined
  );

  $effect(() => {
    if (startDateValue) {
      $form.startDate = toCalendarDate(startDateValue).toString();
    } else {
      $form.startDate = "";
    }
  });

  $effect(() => {
    if (endDateValue) {
      $form.endDate = toCalendarDate(endDateValue).toString();
    } else {
      $form.endDate = undefined;
    }
  });


  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });

  const cardColors = [
    "border-l-2 border-indigo-500",
    "border-l-2 border-teal-500",
    "border-l-2 border-amber-500",
    "border-l-2 border-pink-500",
    "border-l-2 border-emerald-500",
  ];
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
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label>Type</Form.Label>
                    <ToggleGroup.Root
                      type="single"
                      size="lg"
                      {...props}
                      class="col-span-3 gap-1"
                      bind:value={$form.type}
                    >
                      <ToggleGroup.Item value="BORROWED">
                        <ArrowLeftRight /> Borrowed
                      </ToggleGroup.Item>
                      <ToggleGroup.Item value="LENT">
                        <Repeat /> Lent
                      </ToggleGroup.Item>
                    </ToggleGroup.Root>
                    <input type="hidden" name="type" value={$form.type}>
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field form={superFormObj} name="title">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label>Title</Form.Label>
                    <Input
                      {...props}
                      class="col-span-3"
                      bind:value={$form.title}
                    />
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field form={superFormObj} name="repayment">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">Repayment</Form.Label>
                    <ToggleGroup.Root
                      type="single"
                      size="lg"
                      {...props}
                      class="col-span-3 gap-1"
                      bind:value={$form.repayment}
                    >
                      <ToggleGroup.Item value="ONE_TIME">
                        <DollarSign /> One Time
                      </ToggleGroup.Item>
                      <ToggleGroup.Item value="INSTALLMENTS">
                        <CalendarIcon /> Installments
                      </ToggleGroup.Item>
                    </ToggleGroup.Root>
                    <input type="hidden" name="repayment" value={$form.repayment}>
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>

            <Form.Field form={superFormObj} name="amount">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label class="text-right">Amount</Form.Label>
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
            </Form.Field>

            <Form.Field form={superFormObj} name="interestRate">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label>Interest Rate (%)</Form.Label>
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
            </Form.Field>

            <Form.Field form={superFormObj} name="startDate">
              <div class="grid grid-cols-4 items-center gap-4">
                <Form.Control>
                  {#snippet children({ props })}
                    <Form.Label>Start Date</Form.Label>
                    <Popover.Root>
                      <Popover.Trigger
                        class={cn(
                          buttonVariants({
                            variant: "outline",
                            class:
                              "col-span-3 justify-start text-left font-normal",
                          }),
                          !startDateValue && "text-muted-foreground"
                        )}
                      >
                        <CalendarIcon />
                        {startDateValue
                          ? df.format(startDateValue.toDate(getLocalTimeZone()))
                          : "Select a date"}
                      </Popover.Trigger>
                      <Popover.Content class="w-auto p-0">
                        <Calendar
                          bind:value={startDateValue}
                          type="single"
                          initialFocus
                        />
                      </Popover.Content>
                    </Popover.Root>
                  {/snippet}
                </Form.Control>
              </div>
              <Form.FieldErrors />
            </Form.Field>

            {#if $form.repayment === "INSTALLMENTS"}
              <Form.Field form={superFormObj} name="endDate">
                <div class="grid grid-cols-4 items-center gap-4">
                  <Form.Control>
                    {#snippet children({ props })}
                      <Form.Label>End Date</Form.Label>
                      <Popover.Root>
                        <Popover.Trigger
                          class={cn(
                            buttonVariants({
                              variant: "outline",
                              class:
                                "col-span-3 justify-start text-left font-normal",
                            }),
                            !endDateValue && "text-muted-foreground"
                          )}
                        >
                          <CalendarIcon />
                          {endDateValue
                            ? df.format(endDateValue.toDate(getLocalTimeZone()))
                            : "Select a date"}
                        </Popover.Trigger>
                        <Popover.Content class="w-auto p-0">
                          <Calendar
                            bind:value={endDateValue}
                            type="single"
                            initialFocus
                          />
                        </Popover.Content>
                      </Popover.Root>
                    {/snippet}
                  </Form.Control>
                </div>
                <Form.FieldErrors />
              </Form.Field>
            {/if}
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
    {#if data.loans.length === 0}
      <div class="col-span-full text-center mt-24">
        <p class="text-neutral-500">No loans found. Add a new one</p>
      </div>
    {:else}
      {#each data.loans as loan, index}
        <Card class={cardColors[index % cardColors.length]}>
          <CardHeader>
            <CardTitle>{loan.title}</CardTitle>
            <Badge variant="secondary">{loan.type}</Badge>
          </CardHeader>
          <CardContent class="space-y-2">
            <p class="text-lg font-semibold">
              Amount: {loan.amount.toLocaleString()}
            </p>
            {#if loan.interestRate}
              <p class="text-sm text-neutral-600">
                Interest: {loan.interestRate}%
              </p>
            {/if}
            <p class="text-sm text-neutral-600">
              Repayment: {loan.repayment}
            </p>
            <p class="text-sm text-neutral-600">
              Status: {loan.status}
            </p>
            <p class="text-xs text-neutral-500">
              From {loan.startDate.toLocaleDateString()}
              {loan.endDate ? `to ${loan.endDate.toLocaleDateString()}` : ""}
            </p>
          </CardContent>
        </Card>
      {/each}
    {/if}
  </div>
</div>
