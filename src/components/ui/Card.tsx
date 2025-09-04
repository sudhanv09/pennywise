import { type JSX, splitProps } from 'solid-js';
import styles from './Card.module.css';

export interface CardProps {
  children: JSX.Element;
  class?: string;
  variant?: 'default' | 'stat' | 'chart';
}

export function Card(props: CardProps) {
  const [local, others] = splitProps(props, ['children', 'class', 'variant']);
  
  const variant = () => local.variant || 'default';
  const className = () => {
    const baseClass = styles.card;
    const variantClass = styles[`card--${variant()}`];
    const customClass = local.class || '';
    
    return [baseClass, variantClass, customClass].filter(Boolean).join(' ');
  };

  return (
    <div class={className()} {...others}>
      {local.children}
    </div>
  );
}