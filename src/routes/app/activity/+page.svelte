<script lang="ts">
  import Separator from "@/lib/components/ui/separator/separator.svelte";
  import {
    Table,
    TableBody,
    TableRow,
    TableCell,
  } from "$lib/components/ui/table/index.js";
  import { cn } from "$lib/utils";

  type ActivityTransaction = {
    id: string;
    title: string;
    amount: number;
    type: "Expense" | "Income" | "Transfer";
    createdDate: string;
  };

  type ActivityData = {
    transactions: ActivityTransaction[];
  };

  type ActivityGroup = {
    key: string;
    label: string;
    items: ActivityTransaction[];
  };

  let { data } = $props<{ data: ActivityData }>();

  const dateFormatter = new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
  });

  let groupedTransactions = $state<ActivityGroup[]>([]);

  $effect(() => {
    const groups = new Map<string, ActivityGroup>();
    const order: string[] = [];

    for (const transaction of data.transactions) {
      const created = new Date(transaction.createdDate);
      const key = created.toISOString().slice(0, 10);

      if (!groups.has(key)) {
        groups.set(key, {
          key,
          label: dateFormatter.format(created),
          items: [],
        });
        order.push(key);
      }

      groups.get(key)!.items.push(transaction);
    }

    groupedTransactions = order.map((key) => groups.get(key)!);
  });

  const formatAmount = (transaction: ActivityTransaction) => {
    const value = Math.abs(transaction.amount);
    const formatted = value.toLocaleString(undefined, {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });

    if (transaction.type === "Expense") {
      return `- ${formatted}`;
    }

    if (transaction.type === "Income") {
      return `+ ${formatted}`;
    }

    return formatted;
  };

  const amountClass = (transaction: ActivityTransaction) => {
    if (transaction.type === "Expense") {
      return "text-red-500";
    }

    if (transaction.type === "Income") {
      return "text-green-600";
    }

    return "text-neutral-500";
  };
</script>

<div class="container mx-auto mt-12">
  <header class="flex flex-col gap-1">
    <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight first:mt-0">
      Activity
    </h2>
    <p class="text-sm text-neutral-600">
      Transactions from the last 30 days
    </p>
  </header>
  <Separator class="mt-4" />

  {#if data.transactions.length === 0}
    <div class="mt-16 text-center text-sm text-neutral-500">
      No transactions recorded in the last 30 days.
    </div>
  {:else}
    <div class="mt-8 space-y-8">
      {#each groupedTransactions as group (group.key)}
        <section>
          <h3 class="mb-3 text-sm font-medium text-neutral-500">
            {group.label}
          </h3>
          <div class="overflow-hidden rounded-lg border border-neutral-200 bg-white">
            <Table>
              <TableBody class="divide-y divide-neutral-200">
                {#each group.items as transaction (transaction.id)}
                  <TableRow class="text-sm">
                    <TableCell class="px-4 py-3 text-neutral-700">
                      {transaction.title}
                    </TableCell>
                    <TableCell
                      class={cn(
                        "px-4 py-3 text-right font-semibold",
                        amountClass(transaction)
                      )}
                    >
                      {formatAmount(transaction)}
                    </TableCell>
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        </section>
      {/each}
    </div>
  {/if}
</div>