import { type JSX, splitProps } from 'solid-js';
import styles from './Badge.module.css';

export type BadgeVariant = 'default' | 'shopping' | 'bills' | 'groceries' | 'entertainment' | 'transport' | 'other';

export interface BadgeProps {
  children: JSX.Element;
  variant?: BadgeVariant;
  class?: string;
}

export function Badge(props: BadgeProps) {
  const [local, others] = splitProps(props, ['children', 'variant', 'class']);
  
  const variant = () => local.variant || 'default';
  
  const className = () => {
    const baseClass = styles.badge;
    const variantClass = styles[`badge--${variant()}`];
    const customClass = local.class || '';
    
    return [baseClass, variantClass, customClass].filter(Boolean).join(' ');
  };

  return (
    <span class={className()} {...others}>
      {local.children}
    </span>
  );
}