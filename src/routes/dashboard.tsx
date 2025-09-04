import { createFileRoute } from '@tanstack/solid-router';
import { createSignal } from 'solid-js';
import { StatsCard, PieChart, LineChart } from '../components/analytics';
import type { PieChartData, LineChartData } from '../components/analytics';
import './dashboard.css';

export const Route = createFileRoute('/dashboard')({
  component: DashboardComponent,
});

function DashboardComponent() {
  const [financialSummary, setFinancialSummary] = createSignal({
    balance: 2450.75,
    monthlyIncome: 3200.00,
    monthlyExpenses: 1875.25
  });

  const [categoryData, setCategoryData] = createSignal<PieChartData[]>([
    { category: 'Shopping', amount: 450.50, color: '#6366f1' },
    { category: 'Bills', amount: 680.00, color: '#8b5cf6' },
    { category: 'Groceries', amount: 320.75, color: '#06b6d4' },
    { category: 'Entertainment', amount: 180.00, color: '#10b981' },
    { category: 'Transport', amount: 244.00, color: '#f59e0b' }
  ]);

  const [trendData, setTrendData] = createSignal<LineChartData[]>([
    { label: 'Jan', amount: 1650 },
    { label: 'Feb', amount: 1820 },
    { label: 'Mar', amount: 1750 },
    { label: 'Apr', amount: 1920 },
    { label: 'May', amount: 1875 },
    { label: 'Jun', amount: 1875 }
  ]);

  // Calculate trends
  const balanceTrend = () => {
    const income = financialSummary().monthlyIncome;
    const expenses = financialSummary().monthlyExpenses;
    const netChange = ((income - expenses) / expenses) * 100;
    return {
      value: Math.abs(netChange),
      direction: netChange >= 0 ? 'up' as const : 'down' as const
    };
  };

  const expenseTrend = () => {
    const currentMonth = trendData()[trendData().length - 1]?.amount || 0;
    const previousMonth = trendData()[trendData().length - 2]?.amount || 0;
    const change = ((currentMonth - previousMonth) / previousMonth) * 100;
    return {
      value: Math.abs(change),
      direction: change >= 0 ? 'up' as const : 'down' as const
    };
  };

  const incomeTrend = () => ({
    value: 5.2,
    direction: 'up' as const
  });

  return (
    <div class="dashboard">
      <div class="dashboard__header">
        <h1 class="dashboard__title">Financial Dashboard</h1>
        <p class="dashboard__subtitle">Overview of your financial activity</p>
      </div>

      <div class="dashboard__stats">
        <StatsCard
          title="Current Balance"
          value={financialSummary().balance}
          icon="💰"
          variant="balance"
          trend={balanceTrend()}
        />

        <StatsCard
          title="Monthly Income"
          value={financialSummary().monthlyIncome}
          icon="📈"
          variant="income"
          trend={incomeTrend()}
        />

        <StatsCard
          title="Monthly Expenses"
          value={financialSummary().monthlyExpenses}
          icon="📊"
          variant="expense"
          trend={expenseTrend()}
        />
      </div>

      <div class="dashboard__charts">
        <div class="dashboard__chartSection">
          <PieChart
            title="Expenses by Category"
            data={categoryData()}
          />
        </div>

        <div class="dashboard__chartSection">
          <LineChart
            title="Monthly Spending Trend"
            data={trendData()}
            color="#6366f1"
          />
        </div>
      </div>
    </div>
  );
}