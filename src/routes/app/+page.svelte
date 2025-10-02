<script lang="ts">
  import type { PageData } from "./$types";
  import {
    Card,
    CardTitle,
    CardContent,
    CardDescription,
    CardHeader,
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
  import { Wallet, TrendingUp, TrendingDown, Calendar } from "@lucide/svelte";

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

<div class="flex flex-col gap-6 p-6 max-w-7xl mx-auto">
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <Card class="border-2 border-primary/20">
      <CardHeader
        class="flex flex-row items-center justify-between space-y-0 pb-2"
      >
        <CardTitle class="text-sm font-medium">Total Balance</CardTitle>
        <Wallet class="h-4 w-4 text-primary" />
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">
          {formatCurrency(data.totalBalance)}
        </div>
        <p class="text-xs text-muted-foreground mt-1">
          Across {data.accounts.length} account{data.accounts.length !== 1
            ? "s"
            : ""}
        </p>
      </CardContent>
    </Card>

    <!-- Expense Card -->
    <Card class="border-2 border-red-200">
      <CardHeader
        class="flex flex-row items-center justify-between space-y-0 pb-2"
      >
        <CardTitle class="text-sm font-medium">Total Expenses</CardTitle>
        <TrendingDown class="h-4 w-4 text-red-600" />
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold text-red-600">
          {formatCurrency(data.totalExpense)}
        </div>
        <p class="text-xs text-muted-foreground mt-1">This month</p>
      </CardContent>
    </Card>

    <!-- Income Card -->
    <Card class="border-2 border-green-200">
      <CardHeader
        class="flex flex-row items-center justify-between space-y-0 pb-2"
      >
        <CardTitle class="text-sm font-medium">Total Income</CardTitle>
        <TrendingUp class="h-4 w-4 text-green-600" />
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold text-green-600">
          {formatCurrency(data.totalIncome)}
        </div>
        <p class="text-xs text-muted-foreground mt-1">This month</p>
      </CardContent>
    </Card>
  </div>

  <!-- Chart and Bank Accounts Section -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Category Breakdown Pie Chart -->
    <Card class="flex flex-col">
      <CardHeader class="items-center pb-0">
        <CardTitle>Spending by Category</CardTitle>
        <CardDescription>Category-wise expense breakdown</CardDescription>
      </CardHeader>
      <CardContent class="flex-1 pb-0">
        {#if chartData.length > 0}
          <Chart.Container
            config={chartConfig}
            class="mx-auto aspect-square max-h-[300px]"
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
          <div
            class="flex items-center justify-center h-[300px] text-muted-foreground"
          >
            No expense data available
          </div>
        {/if}
      </CardContent>
      <CardContent class="flex-col gap-2 text-sm pt-4">
        {#each data.categoryBreakdown as item, index}
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <div
                class="w-3 h-3 rounded-full"
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
      </CardContent>
    </Card>

    <!-- Bank Accounts -->
    <Card>
      <CardHeader>
        <CardTitle>Bank Accounts</CardTitle>
        <CardDescription>Current balances across accounts</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="space-y-4">
          {#each data.accounts as account}
            <div class="flex items-center justify-between">
              <div>
                <div class="font-medium">{account.name}</div>
                <div class="text-sm text-muted-foreground">
                  {account.currency}
                </div>
              </div>
              <div
                class="text-lg font-bold {account.balance >= 0
                  ? 'text-green-600'
                  : 'text-red-600'}"
              >
                {formatCurrency(account.balance)}
              </div>
            </div>
          {/each}
          {#if data.accounts.length === 0}
            <div class="text-center text-muted-foreground py-8">
              No accounts found
            </div>
          {/if}
        </div>
      </CardContent>
    </Card>
  </div>
</div>

<!-- Recent Transactions Table - Full Width -->
<div class="px-6 pb-6 max-w-7xl mx-auto">
  <div class="mb-4">
    <h2 class="text-2xl font-bold tracking-tight">Recent Transactions</h2>
    <p class="text-muted-foreground">Transactions grouped by date</p>
  </div>

  {#if data.transactionsByDate.length > 0}
    <div class="space-y-6">
      {#each data.transactionsByDate as dateGroup}
        <div>
          <!-- Date Header -->
          <div
            class="flex items-center gap-2 mb-3 px-2 py-2 bg-muted/50 rounded-lg"
          >
            <Calendar class="h-4 w-4 text-muted-foreground" />
            <span class="text-sm font-semibold"
              >{formatDate(dateGroup.date)}</span
            >
          </div>

          <!-- Transactions Table -->
          <div class="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead class="w-[200px]">Category</TableHead>
                  <TableHead>Title</TableHead>
                  <TableHead class="text-right w-[150px]">Amount</TableHead>
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
  {:else}
    <div class="text-center py-12 text-muted-foreground border rounded-lg">
      <p class="text-lg font-medium">No transactions yet</p>
      <p class="text-sm mt-1">Start by adding your first transaction</p>
    </div>
  {/if}
</div>
