import { splitProps } from 'solid-js';
import { Card } from '../ui/Card';
import styles from './StatsCard.module.css';

export interface StatsCardProps {
  title: string;
  value: string | number;
  icon: string;
  variant: 'balance' | 'income' | 'expense';
  trend?: {
    value: number;
    direction: 'up' | 'down';
  };
  class?: string;
}

export function StatsCard(props: StatsCardProps) {
  const [local, others] = splitProps(props, ['title', 'value', 'icon', 'variant', 'trend', 'class']);
  
  const formatValue = () => {
    if (typeof local.value === 'number') {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 0,
        maximumFractionDigits: 2,
      }).format(local.value);
    }
    return local.value;
  };

  const className = () => {
    const baseClass = styles.statsCard;
    const variantClass = styles[`statsCard--${local.variant}`];
    const customClass = local.class || '';
    
    return [baseClass, variantClass, customClass].filter(Boolean).join(' ');
  };

  return (
    <Card variant="stat" class={className()} {...others}>
      <div class={styles.statsCard__header}>
        <div class={styles.statsCard__iconWrapper}>
          <span class={styles.statsCard__icon} innerHTML={local.icon} />
        </div>
        <h3 class={styles.statsCard__title}>{local.title}</h3>
      </div>
      
      <div class={styles.statsCard__content}>
        <div class={styles.statsCard__value}>{formatValue()}</div>
        
        {local.trend && (
          <div class={`${styles.statsCard__trend} ${styles[`statsCard__trend--${local.trend.direction}`]}`}>
            <span class={styles.statsCard__trendIcon}>
              {local.trend.direction === 'up' ? '↗' : '↘'}
            </span>
            <span class={styles.statsCard__trendValue}>
              {Math.abs(local.trend.value)}%
            </span>
          </div>
        )}
      </div>
    </Card>
  );
}