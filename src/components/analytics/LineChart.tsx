import { type JSX, splitProps, createEffect, onMount, onCleanup } from 'solid-js';
import { Card } from '../ui/Card';
import styles from './Chart.module.css';
import {
  Chart,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';

// Register Chart.js components for line chart
Chart.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export interface LineChartData {
  label: string;
  amount: number;
}

export interface LineChartProps {
  data: LineChartData[];
  title?: string;
  color?: string;
  class?: string;
}

export function LineChart(props: LineChartProps) {
  const [local, others] = splitProps(props, ['data', 'title', 'color', 'class']);
  let canvasRef: HTMLCanvasElement | undefined;
  let chartInstance: Chart | null = null;

  const className = () => {
    const baseClass = styles.chart;
    const customClass = local.class || '';
    
    return [baseClass, customClass].filter(Boolean).join(' ');
  };

  const createChart = () => {
    if (!canvasRef || local.data.length === 0) return;

    // Destroy existing chart
    if (chartInstance) {
      chartInstance.destroy();
      chartInstance = null;
    }

    const labels = local.data.map(item => item.label);
    const amounts = local.data.map(item => item.amount);
    const chartColor = local.color || '#6366f1';

    chartInstance = new Chart(canvasRef, {
      type: 'line',
      data: {
        labels,
        datasets: [{
          label: 'Amount',
          data: amounts,
          borderColor: chartColor,
          backgroundColor: chartColor + '20', // Add transparency
          borderWidth: 3,
          fill: true,
          tension: 0.4,
          pointBackgroundColor: chartColor,
          pointBorderColor: '#ffffff',
          pointBorderWidth: 2,
          pointRadius: 6,
          pointHoverRadius: 8,
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: false,
          },
          tooltip: {
            callbacks: {
              label: (context: any) => {
                const value = context.parsed.y;
                return `${context.label}: $${value.toFixed(2)}`;
              }
            }
          }
        },
        scales: {
          y: {
            beginAtZero: true,
            ticks: {
              callback: (value: any) => `$${value}`
            },
            grid: {
              color: '#e5e7eb',
            }
          },
          x: {
            grid: {
              color: '#e5e7eb',
            }
          }
        },
        interaction: {
          intersect: false,
          mode: 'index',
        }
      }
    });
  };

  onMount(() => {
    createChart();
  });

  createEffect(() => {
    local.data;
    local.color;
    createChart();
  });

  onCleanup(() => {
    if (chartInstance) {
      chartInstance.destroy();
      chartInstance = null;
    }
  });

  return (
    <Card variant="chart" class={className()} {...others}>
      {local.title && (
        <div class={styles.chart__header}>
          <h3 class={styles.chart__title}>{local.title}</h3>
        </div>
      )}
      
      <div class={styles.chart__container}>
        <canvas
          ref={canvasRef}
          class={styles.chart__canvas}
        />
        
        {local.data.length === 0 && (
          <div class={styles.chart__empty}>
            <p>No data available</p>
          </div>
        )}
      </div>
    </Card>
  );
}