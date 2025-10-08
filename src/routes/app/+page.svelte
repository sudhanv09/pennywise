<script lang="ts">
  import type { PageData } from "./$types";
  import {
    Card,
    CardTitle,
    CardContent,
    CardDescription,
    CardHeader,
    CardFooter,
  } from "@/lib/components/ui/card";
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from "@/lib/components/ui/table";
  import * as Chart from "$lib/components/ui/chart/index.js";
  import { PieChart } from "layerchart";
  import {
    Wallet,
    TrendingUp,
    TrendingDown,
    Calendar,
    ArrowUpDown,
    Plus,
  } from "@lucide/svelte";
  import Fab from "$lib/components/app/fab.svelte";

  let { data }: { data: PageData } = $props();

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: data.preferredCurrency,
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(amount);
  }

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  }

  const chartColors = [
    "hsl(var(--chart-1))",
    "hsl(var(--chart-2))",
    "hsl(var(--chart-3))",
    "hsl(var(--chart-4))",
    "hsl(var(--chart-5))",
  ];

  const chartData = data.categoryBreakdown.map((item, index) => ({
    category: item.category,
    amount: item.amount,
    color: chartColors[index % chartColors.length],
  }));

  const netCashFlow = data.totalIncome - data.totalExpense;

  const chartConfig = data.categoryBreakdown.reduce(
    (config, item, index) => {
      const key = item.category.toLowerCase().replace(/\s+/g, "_");
      config[key] = {
        label: item.category,
        color: chartColors[index % chartColors.length],
      };
      return config;
    },
    {} as Record<string, { label: string; color: string }>
  );
</script>

<div class="flex w-full flex-col gap-6 px-6 pb-10 pt-6">
  <div class="flex flex-col gap-1">
    <h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">
      Financial Snapshot
    </h1>
    <p class="text-muted-foreground">
      Understand your balances, cash flow, and spending at a glance.
    </p>
  </div>

  <div
    class="grid auto-rows-[minmax(180px,auto)] grid-cols-1 gap-6 lg:grid-cols-6 xl:grid-cols-12"
  >
    <Card
      class="border border-primary/30 bg-primary/5 shadow-sm lg:col-span-3 xl:col-span-3"
    >
      <CardHeader
        class="flex flex-row items-start justify-between space-y-0 pb-2"
      >
        <div class="space-y-1">
          <CardTitle class="text-sm font-medium text-primary">
            Total Balance
          </CardTitle>
          <CardDescription class="text-xs">
            Across {data.accounts.length} account{data.accounts.length !== 1
              ? "s"
              : ""}
          </CardDescription>
        </div>
        <div class="rounded-full bg-primary/10 p-2 text-primary">
          <Wallet class="h-5 w-5" />
        </div>
      </CardHeader>
      <CardContent class="pt-1">
        <div class="text-3xl font-semibold">
          {formatCurrency(data.totalBalance)}
        </div>
      </CardContent>
    </Card>

    <Card
      class="border border-red-200 bg-red-50/80 shadow-sm dark:border-red-900/40 dark:bg-red-950/30 lg:col-span-3 xl:col-span-3"
    >
      <CardHeader
        class="flex flex-row items-start justify-between space-y-0 pb-2"
      >
        <div class="space-y-1">
          <CardTitle class="text-sm font-medium text-red-600 dark:text-red-400">
            Total Expenses
          </CardTitle>
          <CardDescription class="text-xs">This month</CardDescription>
        </div>
        <div
          class="rounded-full bg-red-100 p-2 text-red-600 dark:bg-red-900/40 dark:text-red-400"
        >
          <TrendingDown class="h-5 w-5" />
        </div>
      </CardHeader>
      <CardContent class="pt-1">
        <div class="text-3xl font-semibold text-red-600 dark:text-red-400">
          {formatCurrency(data.totalExpense)}
        </div>
      </CardContent>
    </Card>

    <Card
      class="border border-emerald-200 bg-emerald-50/70 shadow-sm dark:border-emerald-900/40 dark:bg-emerald-950/30 lg:col-span-3 xl:col-span-3"
    >
      <CardHeader
        class="flex flex-row items-start justify-between space-y-0 pb-2"
      >
        <div class="space-y-1">
          <CardTitle class="text-sm font-medium text-emerald-600 dark:text-emerald-400">
            Total Income
          </CardTitle>
          <CardDescription class="text-xs">This month</CardDescription>
        </div>
        <div
          class="rounded-full bg-emerald-100 p-2 text-emerald-600 dark:bg-emerald-900/40 dark:text-emerald-400"
        >
          <TrendingUp class="h-5 w-5" />
        </div>
      </CardHeader>
      <CardContent class="pt-1">
        <div class="text-3xl font-semibold text-emerald-600 dark:text-emerald-400">
          {formatCurrency(data.totalIncome)}
        </div>
      </CardContent>
    </Card>

    <Card
      class="border border-muted-foreground/20 bg-card shadow-sm lg:col-span-3 xl:col-span-3"
    >
      <CardHeader
        class="flex flex-row items-start justify-between space-y-0 pb-2"
      >
        <div class="space-y-1">
          <CardTitle class="text-sm font-medium">Net Cash Flow</CardTitle>
          <CardDescription class="text-xs">
            Income minus expenses
          </CardDescription>
        </div>
        <div class="rounded-full bg-muted/60 p-2 text-muted-foreground">
          <ArrowUpDown class="h-5 w-5" />
        </div>
      </CardHeader>
      <CardContent class="pt-1">
        <div
          class="text-3xl font-semibold {netCashFlow >= 0
            ? 'text-emerald-600'
            : 'text-rose-500'}"
        >
          {netCashFlow >= 0 ? "+" : "-"}
          {formatCurrency(Math.abs(netCashFlow))}
        </div>
      </CardContent>
    </Card>

    <Card class="flex flex-col lg:col-span-4 lg:row-span-2 xl:col-span-6">
      <CardHeader class="space-y-1 pb-0">
        <CardTitle>Recent Transactions</CardTitle>
        <CardDescription>Grouped by posting date.</CardDescription>
      </CardHeader>
      <CardContent class="flex-1 overflow-hidden p-0">
        {#if data.transactionsByDate.length > 0}
          <div class="flex h-full flex-col">
            <div class="flex-1 overflow-y-auto">
              {#each data.transactionsByDate as dateGroup}
                <div class="border-b last:border-b-0">
                  <div class="flex items-center gap-2 bg-muted/70 px-6 py-3">
                    <Calendar class="h-4 w-4 text-muted-foreground" />
                    <span class="text-sm font-semibold">
                      {formatDate(dateGroup.date)}
                    </span>
                  </div>
                  <div class="overflow-x-auto px-6 py-3">
                    <Table class="w-full min-w-[480px]">
                      <TableHeader>
                        <TableRow>
                          <TableHead class="w-[200px]">Category</TableHead>
                          <TableHead>Title</TableHead>
                          <TableHead class="w-[150px] text-right">
                            Amount
                          </TableHead>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        {#each dateGroup.transactions as transaction}
                          <TableRow>
                            <TableCell class="font-medium">
                              {transaction.category}
                            </TableCell>
                            <TableCell>{transaction.title}</TableCell>
                            <TableCell
                              class="text-right font-semibold {transaction.type ===
                              'Expense'
                                ? 'text-red-600'
                                : transaction.type === 'Income'
                                  ? 'text-green-600'
                                  : 'text-blue-600'}"
                            >
                              {transaction.type === "Expense" ? "-" : "+"}
                              {formatCurrency(transaction.amount)}
                            </TableCell>
                          </TableRow>
                        {/each}
                      </TableBody>
                    </Table>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">
            <div class="text-center">
              <p class="text-base font-medium">No transactions yet</p>
              <p class="text-sm">Start by adding your first transaction.</p>
            </div>
          </div>
        {/if}
      </CardContent>
    </Card>

    <Card
      class="flex flex-col lg:col-span-2 lg:col-start-5 xl:col-span-3 xl:col-start-7"
    >
      <CardHeader class="space-y-1 pb-0">
        <CardTitle>Spending by Category</CardTitle>
        <CardDescription>Track where your expenses concentrate.</CardDescription>
      </CardHeader>
      <CardContent class="flex-1 pb-0">
        {#if chartData.length > 0}
          <Chart.Container
            config={chartConfig}
            class="mx-auto aspect-square w-full max-w-[280px]"
          >
            <PieChart
              data={chartData}
              key="category"
              value="amount"
              c="color"
              innerRadius={60}
              padding={10}
              props={{ pie: { motion: "tween" } }}
            >
              {#snippet tooltip()}
                <Chart.Tooltip hideLabel />
              {/snippet}
            </PieChart>
          </Chart.Container>
        {:else}
          <div class="flex h-full items-center justify-center text-sm text-muted-foreground">
            No expense data available yet.
          </div>
        {/if}
      </CardContent>
      {#if chartData.length > 0}
        <CardFooter class="flex flex-col gap-2 pt-4 text-sm">
          {#each data.categoryBreakdown as item, index}
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <div
                  class="h-3 w-3 rounded-full"
                  style="background-color: {chartColors[
                    index % chartColors.length
                  ]}"
                ></div>
                <span>{item.category}</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="font-medium">{formatCurrency(item.amount)}</span>
                <span class="text-muted-foreground">
                  ({item.percentage.toFixed(1)}%)
                </span>
              </div>
            </div>
          {/each}
        </CardFooter>
      {/if}
    </Card>

    <Card
      class="flex flex-col lg:col-span-2 lg:col-start-5 xl:col-span-3 xl:col-start-7"
    >
      <CardHeader class="space-y-1 pb-0">
        <CardTitle>Bank Accounts</CardTitle>
        <CardDescription>Live balances across your accounts.</CardDescription>
      </CardHeader>
      <CardContent class="flex-1 pt-4">
        {#if data.accounts.length > 0}
          <div class="max-h-[320px] overflow-y-auto pr-1">
            <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
              {#each data.accounts as account}
                <div class="rounded-lg border border-border/60 bg-muted/40 p-3">
                  <div class="flex items-center justify-between text-sm font-medium">
                    <span class="truncate">{account.name}</span>
                    <span class="text-xs uppercase text-muted-foreground">
                      {account.currency}
                    </span>
                  </div>
                  <div
                    class="mt-2 text-lg font-semibold {account.balance >= 0
                      ? 'text-emerald-600'
                      : 'text-rose-500'}"
                  >
                    {formatCurrency(account.balance)}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="flex h-full items-center justify-center rounded-lg border border-dashed border-muted-foreground/40 p-6 text-sm text-muted-foreground">
            You don&apos;t have any accounts yet.
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>

<Fab href="/app/new" label="Add transaction" icon={Plus} />
