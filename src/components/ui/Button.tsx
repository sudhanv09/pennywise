import { type JSX, splitProps } from 'solid-js';
import styles from './Button.module.css';

export interface ButtonProps {
  children: JSX.Element;
  variant?: 'primary' | 'secondary' | 'outline';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  onClick?: () => void;
  type?: 'button' | 'submit';
  class?: string;
}

export function Button(props: ButtonProps) {
  const [local, others] = splitProps(props, [
    'children', 
    'variant', 
    'size', 
    'class'
  ]);
  
  const variant = () => local.variant || 'primary';
  const size = () => local.size || 'md';
  
  const className = () => {
    const baseClass = styles.button;
    const variantClass = styles[`button--${variant()}`];
    const sizeClass = styles[`button--${size()}`];
    const customClass = local.class || '';
    
    return [baseClass, variantClass, sizeClass, customClass].filter(Boolean).join(' ');
  };

  return (
    <button class={className()} {...others}>
      {local.children}
    </button>
  );
}