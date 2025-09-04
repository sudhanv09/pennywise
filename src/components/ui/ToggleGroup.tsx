import { type JSX, splitProps, Show, For } from 'solid-js';
import { ToggleGroup as ArkToggleGroup } from '@ark-ui/solid/toggle-group';
import { Field } from '@ark-ui/solid/field';
import styles from './ToggleGroup.module.css';

export interface ToggleGroupOption {
  value: string;
  label: string;
  icon?: JSX.Element;
}

export interface ToggleGroupProps {
  label: string;
  options: ToggleGroupOption[];
  value?: string;
  onChange?: (value: string) => void;
  error?: string;
  required?: boolean;
  class?: string;
  id?: string;
}

export function ToggleGroup(props: ToggleGroupProps) {
  const [local, others] = splitProps(props, [
    'label',
    'options',
    'value',
    'onChange',
    'error',
    'required',
    'class',
    'id'
  ]);
  
  const toggleGroupId = () => local.id || `toggle-group-${Math.random().toString(36).substr(2, 9)}`;
  
  const className = () => {
    const baseClass = styles.field;
    const customClass = local.class || '';
    
    return [baseClass, customClass].filter(Boolean).join(' ');
  };

  const rootClassName = () => {
    const baseClass = styles.root;
    const errorClass = local.error ? styles['root--error'] : '';
    
    return [baseClass, errorClass].filter(Boolean).join(' ');
  };

  return (
    <Field.Root class={className()}>
      <Field.Label class={styles.label} for={toggleGroupId()}>
        {local.label}
        <Show when={local.required}>
          <span class={styles.required}>*</span>
        </Show>
      </Field.Label>
      
      <ArkToggleGroup.Root
        id={toggleGroupId()}
        class={rootClassName()}
        value={local.value ? [local.value] : []}
        onValueChange={(details) => local.onChange?.(details.value[0])}
        {...others}
      >
        <For each={local.options}>
          {(option) => (
            <ArkToggleGroup.Item class={styles.item} value={option.value}>
              <Show when={option.icon}>
                <span class={styles.icon}>
                  {option.icon}
                </span>
              </Show>
              <span class={styles.text}>
                {option.label}
              </span>
            </ArkToggleGroup.Item>
          )}
        </For>
      </ArkToggleGroup.Root>
      
      <Show when={local.error}>
        <Field.ErrorText class={styles.error}>
          {local.error}
        </Field.ErrorText>
      </Show>
    </Field.Root>
  );
}