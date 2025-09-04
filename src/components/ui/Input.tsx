import { splitProps, Show } from 'solid-js';
import { Field } from '@ark-ui/solid/field';
import styles from './Input.module.css';

export interface InputProps {
  label: string;
  placeholder?: string;
  required?: boolean;
  type?: 'text' | 'number' | 'date' | 'email';
  value?: string | number;
  onInput?: (value: string) => void;
  error?: string;
  class?: string;
  id?: string;
}

export function Input(props: InputProps) {
  const [local, others] = splitProps(props, [
    'label',
    'placeholder', 
    'required',
    'type',
    'value',
    'onInput',
    'error',
    'class',
    'id'
  ]);
  
  const inputType = () => local.type || 'text';
  const inputId = () => local.id || `input-${Math.random().toString(36).substr(2, 9)}`;
  
  const className = () => {
    const baseClass = styles.field;
    const customClass = local.class || '';
    
    return [baseClass, customClass].filter(Boolean).join(' ');
  };

  const inputClassName = () => {
    const baseClass = styles.input;
    const errorClass = local.error ? styles['input--error'] : '';
    
    return [baseClass, errorClass].filter(Boolean).join(' ');
  };

  return (
    <Field.Root class={className()}>
      <Field.Label class={styles.label} for={inputId()}>
        {local.label}
        <Show when={local.required}>
          <span class={styles.required}>*</span>
        </Show>
      </Field.Label>
      
      <Field.Input
        id={inputId()}
        class={inputClassName()}
        type={inputType()}
        placeholder={local.placeholder}
        required={local.required}
        value={local.value || ''}
        onInput={(e) => local.onInput?.(e.currentTarget.value)}
        {...others}
      />
      
      <Show when={local.error}>
        <Field.ErrorText class={styles.error}>
          {local.error}
        </Field.ErrorText>
      </Show>
    </Field.Root>
  );
}