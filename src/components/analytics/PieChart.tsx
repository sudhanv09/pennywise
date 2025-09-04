import { type JSX, splitProps, createEffect, onMount, onCleanup } from 'solid-js';
import { Card } from '../ui/Card';
import styles from './Chart.module.css';
import { Chart, ArcElement, Tooltip, Legend } from 'chart.js';

// Register Chart.js components for pie chart
Chart.register(ArcElement, Tooltip, Legend);

export interface PieChartData {
  category: string;
  amount: number;
  color: string;
}

export interface PieChartProps {
  data: PieChartData[];
  title?: string;
  class?: string;
}

export function PieChart(props: PieChartProps) {
  const [local, others] = splitProps(props, ['data', 'title', 'class']);
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

    const labels = local.data.map(item => item.category);
    const amounts = local.data.map(item => item.amount);
    const colors = local.data.map(item => item.color);

    chartInstance = new Chart(canvasRef, {
      type: 'pie',
      data: {
        labels,
        datasets: [{
          data: amounts,
          backgroundColor: colors,
          borderColor: '#ffffff',
          borderWidth: 2,
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: false, // We'll use our custom legend
          },
          tooltip: {
            callbacks: {
              label: (context: any) => {
                const value = context.parsed;
                const total = context.dataset.data.reduce((a: number, b: number) => a + b, 0);
                const percentage = ((value / total) * 100).toFixed(1);
                return `${context.label}: $${value.toFixed(2)} (${percentage}%)`;
              }
            }
          }
        }
      }
    });
  };

  onMount(() => {
    createChart();
  });

  createEffect(() => {
    local.data;
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
      
      {local.data.length > 0 && (
        <div class={styles.chart__legend}>
          {local.data.map((item) => (
            <div class={styles.chart__legendItem}>
              <div 
                class={styles.chart__legendColor}
                style={{ 'background-color': item.color }}
              />
              <span class={styles.chart__legendLabel}>
                {item.category}
              </span>
              <span class={styles.chart__legendValue}>
                ${item.amount.toFixed(0)}
              </span>
            </div>
          ))}
        </div>
      )}
    </Card>
  );
}